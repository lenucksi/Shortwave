// Shortwave - library_page.rs
// Copyright (C) 2021-2024  Felix Häcker <haeckerfelix@gnome.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::cell::{Cell, OnceCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{Properties, clone, subclass};
use gtk::{CompositeTemplate, glib};

use crate::api::{SwStation, SwStationSorter, SwStationSorting, SwStationSortingType};
use crate::app::SwApplication;
use crate::config;
use crate::database::SwLibraryStatus;
use crate::i18n::*;
use crate::settings::{Key, settings_manager};
use crate::ui::{SwApplicationWindow, SwStationDialog, SwStationRow, ToastWindow};

mod imp {
    use super::*;

    #[derive(Default, Debug, Properties, CompositeTemplate)]
    #[template(resource = "/de/haeckerfelix/Shortwave/gtk/library_page.ui")]
    #[properties(wrapper_type = super::SwLibraryPage)]
    pub struct SwLibraryPage {
        #[template_child]
        status_page: TemplateChild<adw::StatusPage>,
        #[template_child]
        stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        gridview: TemplateChild<gtk::GridView>,
        #[template_child]
        select_toggle: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        delete_selected_button: TemplateChild<gtk::Button>,

        selection_mode: Cell<bool>,
        sort_list_model: OnceCell<gtk::SortListModel>,

        #[property(get, set, builder(SwStationSorting::default()))]
        sorting: Cell<SwStationSorting>,
        #[property(get, set, builder(SwStationSortingType::Ascending))]
        sorting_type: Cell<SwStationSortingType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwLibraryPage {
        const NAME: &'static str = "SwLibraryPage";
        type ParentType = adw::NavigationPage;
        type Type = super::SwLibraryPage;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.install_property_action("library.set-sorting", "sorting");
            klass.install_property_action("library.set-sorting-type", "sorting-type");
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SwLibraryPage {
        fn constructed(&self) {
            self.parent_constructed();
            let library = SwApplication::default().library();

            settings_manager::bind_property(Key::LibrarySorting, &*self.obj(), "sorting");
            settings_manager::bind_property(Key::LibrarySortingType, &*self.obj(), "sorting-type");

            let sorter = SwStationSorter::new();
            self.obj()
                .bind_property("sorting", &sorter, "sorting")
                .bidirectional()
                .build();

            self.obj()
                .bind_property("sorting-type", &sorter, "sorting-type")
                .bidirectional()
                .build();

            let sort_list_model =
                gtk::SortListModel::new(Some(library.model()), Some(sorter.clone()));
            self.sort_list_model.set(sort_list_model.clone()).unwrap();

            // Ensure that row type is registered
            SwStationRow::static_type();

            // Station grid view
            let model = gtk::NoSelection::new(Some(sort_list_model));
            self.gridview.set_model(Some(&model));

            self.gridview.connect_activate(clone!(
                #[weak(rename_to = imp)]
                self,
                move |gridview, pos| {
                    if imp.selection_mode.get() {
                        return;
                    }
                    let model = gridview.model().unwrap();
                    let station = model.item(pos).unwrap().downcast::<SwStation>().unwrap();
                    let station_dialog = SwStationDialog::new(&station);
                    station_dialog.present(Some(gridview));
                }
            ));

            self.select_toggle.connect_toggled(clone!(
                #[weak(rename_to = imp)]
                self,
                move |toggle| {
                    if toggle.is_active() {
                        imp.enter_selection_mode();
                    } else {
                        imp.exit_selection_mode();
                    }
                }
            ));

            self.delete_selected_button.connect_clicked(clone!(
                #[weak(rename_to = imp)]
                self,
                move |_| {
                    let imp = imp.clone();
                    glib::spawn_future_local(clone!(
                        #[weak]
                        imp,
                        async move {
                            imp.delete_selected_stations().await;
                        }
                    ));
                }
            ));

            // Setup empty state page
            self.status_page.set_icon_name(Some(*config::APP_ID));

            // Welcome text which gets displayed when the library is empty. "{}" is the
            // application name.
            self.status_page
                .set_title(&i18n_f("Welcome to {}", &[*config::NAME]));

            // Set initial stack page
            self.update_stack_page();

            library.connect_notify_local(
                Some("status"),
                clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |_, _| imp.update_stack_page()
                ),
            );
        }
    }

    impl WidgetImpl for SwLibraryPage {}

    impl NavigationPageImpl for SwLibraryPage {}

    impl SwLibraryPage {
        fn enter_selection_mode(&self) {
            self.selection_mode.set(true);
            self.select_toggle.set_label(&i18n("Done"));

            let sort_list_model = self.sort_list_model.get().unwrap();
            let selection = gtk::MultiSelection::new(Some(sort_list_model.clone()));
            self.gridview.set_model(Some(&selection));

            self.delete_selected_button.set_visible(true);
        }

        fn exit_selection_mode(&self) {
            self.selection_mode.set(false);
            self.select_toggle.set_label(&i18n("Select"));

            if let Some(model) = self.gridview.model() {
                if let Ok(selection) = model.downcast::<gtk::MultiSelection>() {
                    selection.unselect_all();
                }
            }

            let sort_list_model = self.sort_list_model.get().unwrap();
            let model = gtk::NoSelection::new(Some(sort_list_model.clone()));
            self.gridview.set_model(Some(&model));

            self.delete_selected_button.set_visible(false);
        }

        async fn delete_selected_stations(&self) {
            let model = self.gridview.model().unwrap();
            let Some(selection) = model.downcast::<gtk::MultiSelection>().ok() else {
                return;
            };

            let n_items = selection.n_items();
            let mut stations = Vec::new();
            for i in 0..n_items {
                if selection.is_selected(i) {
                    if let Some(item) = selection.item(i) {
                        if let Ok(station) = item.downcast::<SwStation>() {
                            stations.push(station);
                        }
                    }
                }
            }

            if stations.is_empty() {
                return;
            }

            let names: Vec<String> = stations.iter().map(|s| s.metadata().name.clone()).collect();
            let title = if stations.len() == 1 {
                i18n_f("Delete station “{}”?", &[&names[0]])
            } else {
                i18n_f("Delete {} stations?", &[&stations.len().to_string()])
            };

            let body = names
                .iter()
                .take(20)
                .map(|n| format!("• {}", n))
                .collect::<Vec<_>>()
                .join("\n");
            let body = if names.len() > 20 {
                format!("{}\n…", body)
            } else {
                body
            };

            let dialog = adw::AlertDialog::new(Some(&title), Some(&body));
            dialog.add_response("cancel", &i18n("Cancel"));
            dialog.add_response("delete", &i18n("Delete"));
            dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
            dialog.set_default_response(Some("cancel"));
            dialog.set_close_response("cancel");

            let result = dialog.choose_future(Some(&*self.obj())).await;

            if result == "delete" {
                self.select_toggle.set_active(false);

                let library = SwApplication::default().library();
                library.remove_stations(stations);

                let msg = if names.len() == 1 {
                    i18n_f("Deleted “{}”", &[&names[0]])
                } else {
                    i18n_f("Deleted {} stations", &[&names.len().to_string()])
                };

                if let Some(window) = self
                    .obj()
                    .root()
                    .and_then(|r| r.downcast::<SwApplicationWindow>().ok())
                {
                    window.toast_overlay().add_toast(adw::Toast::new(&msg));
                }
            }
        }

        fn update_stack_page(&self) {
            let status = SwApplication::default().library().status();
            match status {
                SwLibraryStatus::Empty => self.stack.set_visible_child_name("empty"),
                SwLibraryStatus::Content => self.stack.set_visible_child_name("content"),
                _ => (),
            }
        }
    }
}

glib::wrapper! {
    pub struct SwLibraryPage(ObjectSubclass<imp::SwLibraryPage>)
        @extends gtk::Widget, adw::NavigationPage,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
