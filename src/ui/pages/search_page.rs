// Shortwave - search_page.rs
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

use std::cell::{Cell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{clone, subclass};
use gtk::{CompositeTemplate, glib};
use indexmap::IndexMap;
use rand::seq::IteratorRandom;

use crate::api::{Error, StationRequest, SwStation, SwStationModel, client};
use crate::discovery::registry::ProviderRegistry;
use crate::discovery::{engine, runner};
use crate::ui::{
    DisplayError, SwDiscoveryResultsDialog, SwDiscoverySourceRow, SwStationDialog, SwStationRow,
    search::SwSearchFilter,
};

mod imp {
    use super::*;

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/de/haeckerfelix/Shortwave/gtk/search_page.ui")]
    pub struct SwSearchPage {
        #[template_child]
        search_filter: TemplateChild<SwSearchFilter>,
        #[template_child]
        stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        popular_flowbox: TemplateChild<gtk::FlowBox>,
        #[template_child]
        random_flowbox: TemplateChild<gtk::FlowBox>,
        #[template_child]
        search_gridview: TemplateChild<gtk::GridView>,
        #[template_child]
        discovery_flowbox: TemplateChild<gtk::FlowBox>,
        #[template_child]
        failure_statuspage: TemplateChild<adw::StatusPage>,

        popular_model: SwStationModel,

        provider_registry: RefCell<Option<ProviderRegistry>>,
        random_model: SwStationModel,
        search_model: SwStationModel,

        loaded: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwSearchPage {
        const NAME: &'static str = "SwSearchPage";
        type ParentType = adw::NavigationPage;
        type Type = super::SwSearchPage;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SwSearchPage {
        fn constructed(&self) {
            self.search_filter.set_sensitive(false);

            // Discover view
            let flowbox_widget_func = |s: &glib::Object| {
                let station: &SwStation = s.downcast_ref().unwrap();
                let row = SwStationRow::new(station);
                let child = gtk::FlowBoxChild::new();
                child.set_child(Some(&row));
                child.into()
            };

            self.popular_flowbox
                .bind_model(Some(&self.popular_model), flowbox_widget_func);
            self.random_flowbox
                .bind_model(Some(&self.random_model), flowbox_widget_func);

            let child_activate_func = |flowbox: &gtk::FlowBox, child: &gtk::FlowBoxChild| {
                let row = child.child().unwrap().downcast::<SwStationRow>().unwrap();
                if let Some(station) = row.station() {
                    let station_dialog = SwStationDialog::new(&station);
                    station_dialog.present(Some(flowbox));
                }
            };

            self.popular_flowbox
                .connect_child_activated(child_activate_func);
            self.random_flowbox
                .connect_child_activated(child_activate_func);

            // Search grid view
            let model = gtk::NoSelection::new(Some(self.search_model.clone()));
            self.search_gridview.set_model(Some(&model));

            self.search_gridview
                .connect_activate(|gv: &gtk::GridView, pos| {
                    let model = gv.model().unwrap();
                    let station = model.item(pos).unwrap().downcast::<SwStation>().unwrap();
                    let station_dialog = SwStationDialog::new(&station);
                    station_dialog.present(Some(gv));
                });

            // Discovery providers
            self.discovery_flowbox.set_max_children_per_line(1);
            self.discovery_flowbox
                .set_selection_mode(gtk::SelectionMode::None);

            let registry = ProviderRegistry::scan();
            let providers = registry.providers().to_vec();
            self.provider_registry.replace(Some(registry));

            for provider in &providers {
                let row = SwDiscoverySourceRow::new(provider);
                let child = gtk::FlowBoxChild::new();
                child.set_child(Some(&row));
                self.discovery_flowbox.append(&child);

                let p = provider.clone();
                let discovery_fb = self.discovery_flowbox.get();
                row.connect_run_provider(move |row| {
                    row.set_loading(true);
                    let p = p.clone();
                    let fb = discovery_fb.clone();
                    let row_clone = row.clone();

                    let (tx, rx) = std::sync::mpsc::channel();
                    let thread_p = p.clone();

                    std::thread::spawn(move || {
                        let engine = engine::create();
                        let result = runner::run_provider(&engine, &thread_p);
                        let _ = tx.send(result);
                    });

                    glib::idle_add_local(move || {
                        match rx.try_recv() {
                            Ok(result) => {
                                row_clone.set_loading(false);
                                match result {
                                    Ok(r) => {
                                        let dialog = SwDiscoveryResultsDialog::new(
                                            &r.stations,
                                        );
                                        dialog.present(Some(&fb));
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "Failed to run discovery provider '{}': {e}",
                                            p.id,
                                        );
                                    }
                                }
                                glib::ControlFlow::Break
                            }
                            Err(std::sync::mpsc::TryRecvError::Empty) => {
                                glib::ControlFlow::Continue
                            }
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                row_clone.set_loading(false);
                                glib::ControlFlow::Break
                            }
                        }
                    });
                });
            }

            self.stack.set_visible_child_name("spinner");
        }
    }

    impl WidgetImpl for SwSearchPage {}

    impl NavigationPageImpl for SwSearchPage {
        fn shown(&self) {
            self.parent_shown();

            if !self.loaded.get() {
                glib::spawn_future_local(clone!(
                    #[weak(rename_to = imp)]
                    self,
                    async move {
                        imp.refresh_discover_page().await;
                    }
                ));
            }

            self.search_filter.grab_focus();
        }
    }

    #[gtk::template_callbacks]
    impl SwSearchPage {
        #[template_callback]
        async fn refresh_discover_page(&self) {
            self.stack.set_visible_child_name("spinner");

            match self.load_discover_stations().await {
                Ok(()) => {
                    self.loaded.set(true);
                    self.search_filter.set_sensitive(true);
                    self.stack.set_visible_child_name("discover");
                    self.search_filter.grab_focus();
                }
                Err(e) => {
                    self.stack.set_visible_child_name("failure");
                    self.failure_statuspage
                        .set_description(Some(&e.to_string()));
                }
            }
        }

        async fn load_discover_stations(&self) -> Result<(), Error> {
            debug!("Update discover stations...");
            let countrycode = Self::region_code().unwrap_or("GB".into());

            // Popular stations
            let request = StationRequest {
                limit: Some(100),
                order: Some("votes".into()),
                reverse: Some(true),
                countrycode: Some(countrycode.clone()),
                ..Default::default()
            };

            let mut stations = client::station_request(request).await?;

            // Anything more than 50k votes can be considered as botted spam
            stations.retain(|_, s| s.metadata().votes < 50_000);

            // Randomize the selection to avoid that always the same stations are visible
            let stations: IndexMap<String, SwStation> = stations
                .iter()
                .sample(&mut rand::rng(), 12)
                .into_iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            self.popular_model.set_stations(stations);

            // Random stations
            let request = StationRequest {
                limit: Some(18),
                order: Some("random".into()),
                countrycode: Some(countrycode),
                ..Default::default()
            };

            let stations = client::station_request(request).await?;
            self.random_model.set_stations(stations);

            Ok(())
        }

        #[template_callback]
        async fn filter_changed(&self) {
            if !self.loaded.get() {
                return;
            }

            // Don't search when no filter is set
            if !self.search_filter.has_filter() {
                self.stack.set_visible_child_name("discover");
                return;
            }

            let request = self.search_filter.station_request();
            self.stack.set_visible_child_name("spinner");

            debug!("Search for: {request:?}");
            let res = client::station_request(request).await;
            res.handle_error("Unable to search for stations");

            if let Ok(stations) = res {
                let no_results = stations.is_empty();
                self.search_model.set_stations(stations);

                if no_results {
                    self.stack.set_visible_child_name("no-results");
                } else {
                    self.stack.set_visible_child_name("results");
                }
            }
        }

        fn region_code() -> Option<String> {
            let locale = sys_locale::get_locale()?;
            let langtag = language_tags::LanguageTag::parse(&locale).ok()?;
            langtag.region().map(|s: &str| s.to_string())
        }
    }
}

glib::wrapper! {
    pub struct SwSearchPage(ObjectSubclass<imp::SwSearchPage>)
        @extends gtk::Widget, adw::NavigationPage,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
