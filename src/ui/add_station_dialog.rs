// Shortwave - station_dialog.rs
// Copyright (C) 2021-2025  Felix Häcker <haeckerfelix@gnome.org>
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

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{Properties, clone, subclass};
use gtk::{CompositeTemplate, gdk, gio, glib};
use url::Url;
use uuid::Uuid;

use crate::api::{StationMetadata, SwStation};
use crate::app::SwApplication;
use crate::i18n::{i18n, i18n_f};
use crate::ui::SwStationCover;

mod imp {
    use super::*;

    #[derive(Debug, Properties, CompositeTemplate)]
    #[template(resource = "/de/haeckerfelix/Shortwave/gtk/add_station_dialog.ui")]
    #[properties(wrapper_type = super::SwAddStationDialog)]
    pub struct SwAddStationDialog {
        #[template_child]
        add_button: TemplateChild<gtk::Button>,
        #[template_child]
        station_cover: TemplateChild<SwStationCover>,
        #[template_child]
        remove_cover_button: TemplateChild<gtk::Button>,
        #[template_child]
        name_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        url_row: TemplateChild<adw::EntryRow>,
        #[template_child]
        pls_status: TemplateChild<gtk::Label>,

        #[property(get)]
        station: SwStation,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwAddStationDialog {
        const NAME: &'static str = "SwAddStationDialog";
        type ParentType = adw::Dialog;
        type Type = super::SwAddStationDialog;

        fn new() -> Self {
            let uuid = Uuid::new_v4().to_string();
            let metadata = StationMetadata::default();
            let station = SwStation::new(&uuid, true, metadata, None);

            Self {
                add_button: TemplateChild::default(),
                station_cover: TemplateChild::default(),
                remove_cover_button: TemplateChild::default(),
                name_row: TemplateChild::default(),
                url_row: TemplateChild::default(),
                pls_status: TemplateChild::default(),
                station,
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SwAddStationDialog {}

    impl WidgetImpl for SwAddStationDialog {}

    impl AdwDialogImpl for SwAddStationDialog {}

    #[gtk::template_callbacks]
    impl SwAddStationDialog {
        #[template_callback]
        fn select_cover_file(&self) {
            let file_chooser = gtk::FileDialog::builder()
                .title(i18n("Select Station Cover"))
                .build();

            let parent = self
                .obj()
                .root()
                .unwrap()
                .downcast::<gtk::Window>()
                .unwrap();

            file_chooser.open(
                Some(&parent),
                gio::Cancellable::NONE,
                clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |res| {
                        match res {
                            Ok(file) => match gdk::Texture::from_file(&file) {
                                Ok(texture) => {
                                    imp.obj().station().set_custom_cover(Some(texture));
                                    imp.remove_cover_button.set_visible(true);
                                }
                                Err(err) => {
                                    error!("Unable to open cover file: {err}");
                                }
                            },
                            Err(err) => error!("Could not get file {err}"),
                        }
                    }
                ),
            );
        }

        #[template_callback]
        fn remove_cover(&self) {
            self.obj().station().set_custom_cover(gdk::Texture::NONE);
            self.remove_cover_button.set_visible(false);
        }

        #[template_callback]
        fn add_station(&self) {
            SwApplication::default()
                .library()
                .add_station(self.obj().station());

            self.obj().close();
        }

        #[template_callback]
        fn update_metadata(&self) {
            let name = self.name_row.text().to_string();
            let has_name = !name.is_empty();
            let url = Url::parse(&self.url_row.text()).ok();

            match url {
                Some(_) => {
                    self.url_row.remove_css_class("error");
                    self.add_button.set_sensitive(has_name);
                }
                None => {
                    self.url_row.add_css_class("error");
                    self.add_button.set_sensitive(false);
                }
            }

            let metadata = StationMetadata {
                name,
                url: url.clone(),
                ..Default::default()
            };
            self.obj().station().set_metadata(metadata);

            if let Some(ref url) = url {
                let path = url.path().to_lowercase();
                if path.ends_with(".pls") || path.ends_with(".m3u") || path.ends_with(".m3u8") {
                    self.pls_status.set_text(&i18n("Fetching playlist..."));
                    self.pls_status.remove_css_class("error");
                    self.pls_status.set_visible(true);
                    self.add_button.set_sensitive(false);

                    let obj = self.obj().clone();
                    let url = url.clone();
                    glib::spawn_future_local(async move {
                        match crate::playlist::fetch_and_parse(&url).await {
                            Ok(entries) => {
                                if entries.is_empty() {
                                    return;
                                }

                                let first = &entries[0];
                                let mut metadata = obj.station().metadata();
                                metadata.url = Some(first.url.clone());
                                metadata.alternate_urls =
                                    entries.iter().skip(1).map(|e| e.url.clone()).collect();
                                metadata.playlist_url = Some(url.clone());

                                if let Some(ref title) = first.title
                                    && obj.station().metadata().name.is_empty()
                                {
                                    metadata.name = title.clone();
                                }

                                obj.station().set_metadata(metadata);

                                let count = entries.len();
                                let msg =
                                    i18n_f("Found {numStreams} stream URLs", &[&count.to_string()]);
                                let imp = imp::SwAddStationDialog::from_obj(&obj);
                                imp.pls_status.set_text(&msg);
                            }
                            Err(e) => {
                                let imp = imp::SwAddStationDialog::from_obj(&obj);
                                imp.pls_status.add_css_class("error");
                                imp.pls_status.set_text(&i18n_f(
                                    "Failed to fetch playlist: {error}",
                                    &[&e.to_string()],
                                ));
                            }
                        }
                    });
                    return;
                }
            }

            self.pls_status.set_visible(false);
        }
    }
}

glib::wrapper! {
    pub struct SwAddStationDialog(ObjectSubclass<imp::SwAddStationDialog>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SwAddStationDialog {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for SwAddStationDialog {
    fn default() -> Self {
        Self::new()
    }
}
