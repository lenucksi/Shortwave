// Shortwave - search_page.rs
// Copyright (C) 2021-2025  Felix Haecker <haeckerfelix@gnome.org>
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
use std::collections::HashSet;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{clone, subclass};
use gtk::{CompositeTemplate, gio, glib};
use indexmap::IndexMap;
use rand::seq::IteratorRandom;

use crate::api::{Error, StationRequest, SwStation, SwStationModel, client};
use crate::discovery::registry::ProviderRegistry;
use crate::discovery::{engine, runner};
use crate::ui::{
    DisplayError, SwDiscoveryResultsDialog, SwDiscoverySourceRow, SwStationDialog, SwStationRow,
    search::SwSearchFilter,
};

const DISABLED_STATE_FILE: &str = ".disabled.json";

fn disabled_state_path() -> std::path::PathBuf {
    ProviderRegistry::user_dir().join(DISABLED_STATE_FILE)
}

fn load_disabled_state() -> HashSet<String> {
    let path = disabled_state_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<String>>(&s).ok())
        .map(|v| v.into_iter().collect())
        .unwrap_or_default()
}

fn save_disabled_state(disabled: &HashSet<String>) {
    let path = disabled_state_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let ids: Vec<&String> = disabled.iter().collect();
    if let Ok(json) = serde_json::to_string(&ids) {
        let _ = std::fs::write(&path, json);
    }
}

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
        #[template_child]
        add_provider_button: TemplateChild<gtk::Button>,
        #[template_child]
        open_provider_folder_button: TemplateChild<gtk::Button>,

        popular_model: SwStationModel,

        provider_registry: RefCell<Option<ProviderRegistry>>,
        disabled_providers: RefCell<HashSet<String>>,
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
            self.build_discovery_providers();
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

        #[template_callback]
        async fn add_provider_clicked(&self) {
            let filter = gtk::FileFilter::new();
            filter.set_name(Some("Rhai Scripts"));
            filter.add_pattern("*.rhai");

            let filter_list = gio::ListStore::new::<gtk::FileFilter>();
            filter_list.append(&filter);

            let dialog = gtk::FileDialog::new();
            dialog.set_title("Select Discovery Script");
            dialog.set_default_filter(Some(&filter));
            dialog.set_filters(Some(&filter_list));

            let future = dialog.open_future(None::<&gtk::Window>);
            match future.await {
                Ok(file) => {
                    if let Some(path) = file.path() {
                        let user_dir = ProviderRegistry::user_dir();
                        let _ = std::fs::create_dir_all(&user_dir);

                        let filename = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let dest = user_dir.join(&filename);

                        match std::fs::copy(&path, &dest) {
                            Ok(_) => {
                                self.rebuild_discovery_providers();
                            }
                            Err(e) => {
                                log::error!("Failed to copy provider script: {e}");
                            }
                        }
                    }
                }
                Err(e) => {
                    if !e.matches(gtk::DialogError::Dismissed) {
                        log::error!("File dialog error: {e}");
                    }
                }
            }
        }

        #[template_callback]
        async fn open_provider_folder_clicked(&self) {
            let user_dir = ProviderRegistry::user_dir();
            let _ = std::fs::create_dir_all(&user_dir);
            let uri = format!("file://{}", user_dir.display());
            let launcher = gtk::UriLauncher::new(&uri);
            let parent = self
                .obj()
                .root()
                .and_then(|r| r.downcast::<gtk::Window>().ok());
            if let Err(e) = launcher.launch_future(parent.as_ref()).await {
                log::error!("Failed to open provider folder: {e}");
            }
        }

        fn region_code() -> Option<String> {
            let locale = sys_locale::get_locale()?;
            let langtag = language_tags::LanguageTag::parse(&locale).ok()?;
            langtag.region().map(|s: &str| s.to_string())
        }

        fn build_discovery_providers(&self) {
            // Clear existing children
            while let Some(child) = self.discovery_flowbox.first_child() {
                self.discovery_flowbox.remove(&child);
            }

            let disabled = load_disabled_state();
            *self.disabled_providers.borrow_mut() = disabled.clone();

            let registry = ProviderRegistry::scan();
            let providers = registry.providers().to_vec();
            self.provider_registry.replace(Some(registry));

            for provider in &providers {
                let row = SwDiscoverySourceRow::new(provider);
                let is_disabled = disabled.contains(&provider.id);
                if is_disabled {
                    row.set_enabled(false);
                }

                // Only show remove button for user providers
                if !row.is_user_provider() {
                    row.imp().remove_button.set_visible(false);
                }

                let child = gtk::FlowBoxChild::new();
                child.set_child(Some(&row));
                self.discovery_flowbox.append(&child);

                let p = provider.clone();
                let discovery_fb = self.discovery_flowbox.get();
                row.connect_run_provider(move | row | {
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
                                        let dialog = SwDiscoveryResultsDialog::new(&r.stations);
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

                let provider_id = provider.id.clone();
                let obj = self.obj().downgrade();
                row.connect_remove_provider(move |_row| {
                    let p_id = provider_id.clone();
                    if let Some(obj) = obj.upgrade() {
                        glib::spawn_future_local(async move {
                            if let Err(e) = ProviderRegistry::remove_provider(&p_id) {
                                log::error!("Failed to remove provider '{p_id}': {e}");
                                return;
                            }
                            obj.imp().rebuild_discovery_providers();
                        });
                    }
                });

                let provider_id = provider.id.clone();
                let obj = self.obj().downgrade();
                row.connect_notify_local(Some("provider-enabled"), move |row, _| {
                    if let Some(obj) = obj.upgrade() {
                        let mut disabled = obj.imp().disabled_providers.borrow_mut();
                        if row.is_enabled() {
                            disabled.remove(&provider_id);
                        } else {
                            disabled.insert(provider_id.clone());
                        }
                        save_disabled_state(&disabled);
                    }
                });
            }
        }

        fn rebuild_discovery_providers(&self) {
            self.build_discovery_providers();
        }
    }
}

glib::wrapper! {
    pub struct SwSearchPage(ObjectSubclass<imp::SwSearchPage>)
        @extends gtk::Widget, adw::NavigationPage,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
