use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::{CompositeTemplate, TemplateChild, gio, glib};

use crate::api::SwStation;
use crate::app::SwApplication;
use crate::discovery::types::StationData;
use crate::playlist::fetch_and_parse;
use url::Url;

mod imp {
    use super::*;
    use std::cell::RefCell;

    use glib::subclass;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/de/haeckerfelix/Shortwave/gtk/discovery_results_dialog.ui")]
    pub struct SwDiscoveryResultsDialog {
        #[template_child]
        pub import_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub cancel_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub stations_list: TemplateChild<gtk::ListView>,
        #[template_child]
        pub import_progress_label: TemplateChild<gtk::Label>,

        pub stations: RefCell<Vec<StationData>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwDiscoveryResultsDialog {
        const NAME: &'static str = "SwDiscoveryResultsDialog";
        type ParentType = adw::Dialog;
        type Type = super::SwDiscoveryResultsDialog;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SwDiscoveryResultsDialog {}

    impl WidgetImpl for SwDiscoveryResultsDialog {}

    impl AdwDialogImpl for SwDiscoveryResultsDialog {}

    #[gtk::template_callbacks]
    impl SwDiscoveryResultsDialog {
        #[template_callback]
        fn import_clicked(&self) {
            let stations = self.stations.borrow().clone();
            if stations.is_empty() {
                return;
            }
            let total = stations.len();

            let body = stations
                .iter()
                .map(|s| format!("• {}", s.name))
                .collect::<Vec<_>>()
                .join("\n");

            let obj = self.obj();

            glib::spawn_future_local(clone!(
                #[weak]
                obj,
                async move {
                    let dialog = adw::AlertDialog::new(
                        Some(&format!("Import these {} stations?", total)),
                        Some(&body),
                    );
                    dialog.add_response("cancel", "_Cancel");
                    dialog.add_response("import", "_Import");
                    dialog.set_response_appearance("import", adw::ResponseAppearance::Suggested);
                    dialog.set_default_response(Some("import"));
                    dialog.set_close_response("cancel");

                    let response = dialog.choose_future(Some(&obj)).await;
                    if response != "import" {
                        return;
                    }

                    let imp = obj.imp();
                    imp.import_progress_label.set_visible(true);
                    imp.import_button.set_sensitive(false);

                    let app = SwApplication::default();

                    for (i, station) in stations.iter().enumerate() {
                        let progress = format!("Importing {} / {}: {}", i + 1, total, station.name);
                        imp.import_progress_label.set_label(&progress);

                        let is_playlist = station.stream_url.ends_with(".pls")
                            || station.stream_url.ends_with(".m3u")
                            || station.stream_url.ends_with(".m3u8");

                        let (url, alternate_urls, playlist_url) = if is_playlist {
                            let pls_url = Url::parse(&station.stream_url).ok();
                            let mut resolved_url = None;
                            let mut alt_urls = Vec::new();
                            if let Some(ref pu) = pls_url
                                && let Ok(entries) = fetch_and_parse(pu).await
                            {
                                let urls: Vec<Url> = entries.into_iter().map(|e| e.url).collect();
                                let mut iter = urls.into_iter();
                                resolved_url = iter.next();
                                alt_urls = iter.collect();
                            }
                            (resolved_url, alt_urls, pls_url)
                        } else {
                            let (chosen, alts) = select_best_format(station);
                            (Url::parse(&chosen).ok(), alts, None)
                        };

                        let metadata = crate::api::StationMetadata {
                            name: station.name.clone(),
                            url,
                            alternate_urls,
                            playlist_url,
                            homepage: station.homepage.as_ref().and_then(|h| Url::parse(h).ok()),
                            favicon: station.icon_url.as_ref().and_then(|i| Url::parse(i).ok()),
                            tags: station.tags.clone().unwrap_or_default(),
                            country: station.country.clone().unwrap_or_default(),
                            language: station.language.clone().unwrap_or_default(),
                            ..Default::default()
                        };

                        let sw_station =
                            SwStation::new(&uuid::Uuid::new_v4().to_string(), true, metadata, None);
                        app.library().add_station(sw_station);
                    }

                    let toast = adw::Toast::new(&crate::i18n::i18n_f(
                        "Imported {} stations",
                        &[&total.to_string()],
                    ));
                    imp.toast_overlay.add_toast(toast);
                    obj.close();
                }
            ));
        }

        #[template_callback]
        fn cancel_clicked(&self) {
            self.obj().close();
        }
    }
}

glib::wrapper! {
    pub struct SwDiscoveryResultsDialog(ObjectSubclass<imp::SwDiscoveryResultsDialog>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SwDiscoveryResultsDialog {
    pub fn new(stations: &[StationData]) -> Self {
        let obj: Self = glib::Object::builder().build();
        let imp = obj.imp();

        obj.set_size_request(500, 500);

        *imp.stations.borrow_mut() = stations.to_vec();

        let store = gio::ListStore::new::<SwStation>();
        for station in stations.iter() {
            store.append(&station.to_sw_station());
        }

        let no_selection = gtk::NoSelection::new(Some(store));
        imp.stations_list.set_model(Some(&no_selection));

        imp.import_button.set_sensitive(!stations.is_empty());

        obj
    }
}

fn select_best_format(station: &StationData) -> (String, Vec<Url>) {
    if station.stream_urls.is_empty() {
        return (station.stream_url.clone(), Vec::new());
    }

    let mut candidates: Vec<&crate::discovery::types::StreamUrlInfo> =
        station.stream_urls.iter().collect();
    candidates.sort_by(|a, b| {
        let a_tls = a.tls.unwrap_or(false);
        let b_tls = b.tls.unwrap_or(false);
        b_tls
            .cmp(&a_tls)
            .then_with(|| {
                let a_codec = a.codec.as_deref().unwrap_or("").to_lowercase();
                let b_codec = b.codec.as_deref().unwrap_or("").to_lowercase();
                let a_is_aac = a_codec.contains("aac");
                let b_is_aac = b_codec.contains("aac");
                b_is_aac.cmp(&a_is_aac)
            })
            .then_with(|| {
                let a_bitrate = a.bitrate.unwrap_or(0);
                let b_bitrate = b.bitrate.unwrap_or(0);
                b_bitrate.cmp(&a_bitrate)
            })
    });

    let best = candidates[0];
    let chosen = best.url.clone();
    let alts: Vec<Url> = candidates[1..]
        .iter()
        .filter_map(|u| Url::parse(&u.url).ok())
        .collect();

    (chosen, alts)
}

trait ToSwStation {
    fn to_sw_station(&self) -> SwStation;
}

impl ToSwStation for StationData {
    fn to_sw_station(&self) -> SwStation {
        SwStation::new(
            &uuid::Uuid::new_v4().to_string(),
            true,
            crate::api::StationMetadata {
                name: self.name.clone(),
                url: url::Url::parse(&self.stream_url).ok(),
                url_resolved: None,
                alternate_urls: self
                    .stream_urls
                    .iter()
                    .filter_map(|u| url::Url::parse(&u.url).ok())
                    .collect(),
                homepage: self.homepage.as_ref().and_then(|h| url::Url::parse(h).ok()),
                favicon: self.icon_url.as_ref().and_then(|i| url::Url::parse(i).ok()),
                tags: self.tags.clone().unwrap_or_default(),
                country: self.country.clone().unwrap_or_default(),
                language: self.language.clone().unwrap_or_default(),
                ..Default::default()
            },
            None,
        )
    }
}
