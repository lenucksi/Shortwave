// Shortwave - discovery_results_dialog.rs
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
use glib::clone;
use gtk::{CompositeTemplate, TemplateChild, gdk, gio, glib, pango};
use url::Url;

use crate::api::SwStation;
use crate::app::SwApplication;
use crate::discovery::types::StationData;
use crate::playlist::fetch_and_parse;

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
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub details_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub details_name: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_country: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_tags: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_url_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_url: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_homepage_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_homepage: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_placeholder: TemplateChild<gtk::Label>,

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
            let model = self.stations_list.model().unwrap();
            let selection = model.downcast::<gtk::MultiSelection>().unwrap();

            let all_stations = self.stations.borrow();
            let mut selected_stations = Vec::new();

            for i in 0..selection.n_items() {
                if selection.is_selected(i)
                    && let Some(item) = selection.item(i)
                        && let Ok(sw_station) = item.downcast::<SwStation>() {
                            let meta = sw_station.metadata();
                            if let Some(sd) = all_stations.iter().find(|s| s.name == meta.name) {
                                selected_stations.push(sd.clone());
                            }
                        }
            }

            drop(all_stations);

            if selected_stations.is_empty() {
                return;
            }
            let total = selected_stations.len();

            let body = selected_stations
                .iter()
                .map(|s| format!("• {}", s.name))
                .collect::<Vec<_>>()
                .join("\n");

            let obj = self.obj();

            glib::spawn_future_local(clone!(
                #[weak]
                obj,
                #[strong]
                selected_stations,
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

                    for (i, station) in selected_stations.iter().enumerate() {
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

        obj.set_size_request(700, 500);

        *imp.stations.borrow_mut() = stations.to_vec();

        let store = gio::ListStore::new::<SwStation>();
        for station in stations.iter() {
            if !station.name.is_empty() && !station.stream_url.is_empty() {
                store.append(&station.to_sw_station());
            }
        }

        let filter = gtk::CustomFilter::new(|_| true);
        let filter_model = gtk::FilterListModel::new(Some(store), Some(filter));

        let selection = gtk::MultiSelection::new(Some(filter_model.clone()));
        imp.stations_list.set_model(Some(&selection));

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_factory, item| {
            let list_item = item.downcast_ref::<gtk::ListItem>().unwrap();

            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
            vbox.set_margin_start(8);
            vbox.set_margin_end(8);
            vbox.set_margin_top(4);
            vbox.set_margin_bottom(4);

            let name = gtk::Label::new(None);
            name.set_halign(gtk::Align::Start);
            name.set_ellipsize(pango::EllipsizeMode::End);
            name.set_css_classes(&["heading"]);

            let subtitle = gtk::Label::new(None);
            subtitle.set_halign(gtk::Align::Start);
            subtitle.set_ellipsize(pango::EllipsizeMode::End);
            subtitle.set_css_classes(&["dim-label", "caption"]);

            vbox.append(&name);
            vbox.append(&subtitle);

            list_item.set_child(Some(&vbox));
        });

        factory.connect_bind(|_factory, item| {
            let list_item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let station = list_item
                .item()
                .and_then(|o| o.downcast::<SwStation>().ok())
                .unwrap();

            let vbox = list_item.child().unwrap().downcast::<gtk::Box>().unwrap();
            let name_label = vbox
                .first_child()
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap();
            let subtitle_label = name_label
                .next_sibling()
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap();

            let meta = station.metadata();
            name_label.set_text(&meta.name);

            let mut subtitle = meta.country.clone();
            if !meta.tags.is_empty() {
                if !subtitle.is_empty() {
                    subtitle.push_str("  •  ");
                }
                subtitle.push_str(&meta.tags);
            }
            subtitle_label.set_text(&subtitle);
        });

        imp.stations_list.set_factory(Some(&factory));

        let details_icon = imp.details_icon.clone();
        let details_name = imp.details_name.clone();
        let details_country = imp.details_country.clone();
        let details_tags = imp.details_tags.clone();
        let details_url_label = imp.details_url_label.clone();
        let details_url = imp.details_url.clone();
        let details_homepage_label = imp.details_homepage_label.clone();
        let details_homepage = imp.details_homepage.clone();
        let details_placeholder = imp.details_placeholder.clone();
        let import_button = imp.import_button.clone();

        selection.connect_selection_changed(move |sel, _pos, _n_items| {
            let count = sel.n_items();
            let n_selected: u32 = (0..count).filter(|i| sel.is_selected(*i)).count() as u32;

            import_button.set_sensitive(n_selected > 0);

            details_icon.set_visible(false);
            details_name.set_visible(false);
            details_country.set_visible(false);
            details_tags.set_visible(false);
            details_url_label.set_visible(false);
            details_url.set_visible(false);
            details_homepage_label.set_visible(false);
            details_homepage.set_visible(false);

            if n_selected == 0 {
                details_placeholder.set_text("Select stations");
                details_placeholder.set_visible(true);
                return;
            }

            if n_selected == 1 {
                for i in 0..count {
                    if sel.is_selected(i) {
                        if let Some(item) = sel.item(i)
                            && let Ok(station) = item.downcast::<SwStation>() {
                                details_placeholder.set_visible(false);

                                let meta = station.metadata();

                                details_name.set_text(&meta.name);
                                details_name.set_visible(true);

                                if !meta.country.is_empty() {
                                    details_country.set_text(&meta.country);
                                    details_country.set_visible(true);
                                }

                                if !meta.tags.is_empty() {
                                    details_tags.set_text(&meta.tags);
                                    details_tags.set_visible(true);
                                }

                                if let Some(ref url) = meta.url {
                                    details_url.set_text(url.as_ref());
                                    details_url.set_visible(true);
                                    details_url_label.set_visible(true);
                                }

                                if let Some(ref hp) = meta.homepage {
                                    details_homepage.set_text(hp.as_ref());
                                    details_homepage.set_visible(true);
                                    details_homepage_label.set_visible(true);
                                }

                                if let Some(ref favicon) = meta.favicon {
                                    let url = favicon.clone();
                                    let icon = details_icon.clone();
                                    glib::spawn_future_local(async move {
                                        if let Ok(resp) = crate::api::http::get(url).await
                                            && let Ok(bytes) = resp.bytes().await {
                                                let gbytes = glib::Bytes::from(bytes.as_ref());
                                                if let Ok(texture) =
                                                    gdk::Texture::from_bytes(&gbytes)
                                                {
                                                    icon.set_paintable(Some(&texture));
                                                    icon.set_visible(true);
                                                }
                                            }
                                    });
                                }
                            }
                        break;
                    }
                }
            } else {
                details_placeholder.set_text(&format!("{} stations selected", n_selected));
                details_placeholder.set_visible(true);
            }
        });

        imp.search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            let filter = gtk::CustomFilter::new(move |obj| {
                let station = obj.downcast_ref::<SwStation>().unwrap();
                if query.is_empty() {
                    return true;
                }
                let meta = station.metadata();
                let haystack = format!(
                    "{} {} {} {}",
                    meta.name.to_lowercase(),
                    meta.country.to_lowercase(),
                    meta.tags.to_lowercase(),
                    meta.language.to_lowercase(),
                );
                haystack.contains(&query.to_lowercase())
            });
            filter_model.set_filter(Some(&filter));
        });

        imp.import_button.set_sensitive(false);

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
