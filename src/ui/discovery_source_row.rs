use std::cell::RefCell;
use std::sync::LazyLock;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{ParamSpec, ParamSpecString, Value, subclass};
use gtk::{CompositeTemplate, TemplateChild, glib};

use crate::discovery::provider::DiscoveryProvider;

mod imp {
    use super::*;

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/de/haeckerfelix/Shortwave/gtk/discovery_source_row.ui")]
    pub struct SwDiscoverySourceRow {
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub description_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub loading_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub run_button: TemplateChild<gtk::Button>,

        pub provider_id: RefCell<String>,
        pub provider_name: RefCell<String>,
        pub provider_description: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwDiscoverySourceRow {
        const NAME: &'static str = "SwDiscoverySourceRow";
        type ParentType = adw::Bin;
        type Type = super::SwDiscoverySourceRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SwDiscoverySourceRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: LazyLock<Vec<Signal>> =
                LazyLock::new(|| vec![Signal::builder("run-provider").build()]);
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: LazyLock<Vec<ParamSpec>> = LazyLock::new(|| {
                vec![
                    ParamSpecString::builder("provider-id").build(),
                    ParamSpecString::builder("provider-name").build(),
                    ParamSpecString::builder("provider-description").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "provider-id" => {
                    *self.provider_id.borrow_mut() = value.get().unwrap_or_default();
                }
                "provider-name" => {
                    *self.provider_name.borrow_mut() = value.get().unwrap_or_default();
                }
                "provider-description" => {
                    *self.provider_description.borrow_mut() = value.get().unwrap_or_default();
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "provider-id" => self.provider_id.borrow().to_value(),
                "provider-name" => self.provider_name.borrow().to_value(),
                "provider-description" => self.provider_description.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl WidgetImpl for SwDiscoverySourceRow {}

    impl BinImpl for SwDiscoverySourceRow {}

    #[gtk::template_callbacks]
    impl SwDiscoverySourceRow {
        #[template_callback]
        async fn run_clicked(&self) {
            self.obj().emit_by_name::<()>("run-provider", &[]);
        }
    }
}

glib::wrapper! {
    pub struct SwDiscoverySourceRow(ObjectSubclass<imp::SwDiscoverySourceRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SwDiscoverySourceRow {
    pub fn new(provider: &DiscoveryProvider) -> Self {
        let obj: Self = glib::Object::builder()
            .property("provider-id", provider.id.clone())
            .property("provider-name", provider.name.clone())
            .property("provider-description", provider.description.clone())
            .build();
        obj.imp()
            .name_label
            .set_markup(&format!("<b>{}</b>", provider.name));
        if provider.description.is_empty() {
            obj.imp().description_label.set_visible(false);
        } else {
            obj.imp().description_label.set_label(&provider.description);
            obj.imp().description_label.set_visible(true);
        }
        obj
    }

    pub fn provider_id(&self) -> String {
        self.imp().provider_id.borrow().clone()
    }

    pub fn set_loading(&self, loading: bool) {
        self.imp().loading_spinner.set_spinning(loading);
        self.imp().loading_spinner.set_visible(loading);
        self.imp().run_button.set_sensitive(!loading);
    }

    pub fn connect_run_provider<F: Fn(&SwDiscoverySourceRow) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        let obj = self.clone();
        self.connect_local("run-provider", true, move |_: &[glib::Value]| {
            f(&obj);
            None
        })
    }
}
