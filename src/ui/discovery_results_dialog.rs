use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{CompositeTemplate, TemplateChild, gio, glib};

use crate::api::SwStation;
use crate::app::SwApplication;
use crate::discovery::types::StationData;

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
        pub stations_list: TemplateChild<gtk::ListView>,

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
            let stations = self.stations.borrow();
            let app = SwApplication::default();
            for station in stations.iter() {
                let station = station.to_sw_station();
                app.library().add_station(station);
            }
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
