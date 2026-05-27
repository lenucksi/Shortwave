use std::cell::RefCell;
use std::sync::LazyLock;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::Signal;
use glib::{ParamSpec, ParamSpecBoolean, ParamSpecString, Value, subclass};
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
        #[template_child]
        pub remove_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub enable_switch: TemplateChild<gtk::Switch>,

        pub provider_id: RefCell<String>,
        pub provider_name: RefCell<String>,
        pub provider_description: RefCell<String>,
        pub provider_enabled: RefCell<bool>,
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
            static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
                vec![
                    Signal::builder("run-provider").build(),
                    Signal::builder("remove-provider").build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: LazyLock<Vec<ParamSpec>> = LazyLock::new(|| {
                vec![
                    ParamSpecString::builder("provider-id").build(),
                    ParamSpecString::builder("provider-name").build(),
                    ParamSpecString::builder("provider-description").build(),
                    ParamSpecBoolean::builder("provider-enabled")
                        .default_value(true)
                        .build(),
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
                "provider-enabled" => {
                    *self.provider_enabled.borrow_mut() = value.get().unwrap_or(true);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "provider-id" => self.provider_id.borrow().to_value(),
                "provider-name" => self.provider_name.borrow().to_value(),
                "provider-description" => self.provider_description.borrow().to_value(),
                "provider-enabled" => self.provider_enabled.borrow().to_value(),
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
            if *self.provider_enabled.borrow() {
                self.obj().emit_by_name::<()>("run-provider", &[]);
            }
        }

        #[template_callback]
        async fn remove_clicked(&self) {
            self.obj().emit_by_name::<()>("remove-provider", &[]);
        }

        #[template_callback]
        async fn enable_toggled(&self) {
            let enabled = self.enable_switch.is_active();
            *self.provider_enabled.borrow_mut() = enabled;
            self.run_button.set_sensitive(enabled);
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
            .property("provider-enabled", provider.enabled)
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
        obj.imp().enable_switch.set_active(provider.enabled);
        obj.imp().run_button.set_sensitive(provider.enabled);
        obj
    }

    pub fn provider_id(&self) -> String {
        self.imp().provider_id.borrow().clone()
    }

    pub fn is_user_provider(&self) -> bool {
        let user_dir = crate::discovery::registry::ProviderRegistry::user_dir();
        let id = self.provider_id();
        user_dir.join(format!("{id}.rhai")).exists()
    }

    pub fn set_loading(&self, loading: bool) {
        self.imp().loading_spinner.set_spinning(loading);
        self.imp().loading_spinner.set_visible(loading);
        self.imp()
            .run_button
            .set_sensitive(!loading && *self.imp().provider_enabled.borrow());
    }

    pub fn set_enabled(&self, enabled: bool) {
        *self.imp().provider_enabled.borrow_mut() = enabled;
        self.imp().enable_switch.set_active(enabled);
        self.imp().run_button.set_sensitive(enabled);
    }

    pub fn is_enabled(&self) -> bool {
        *self.imp().provider_enabled.borrow()
    }

    pub fn set_remove_visible(&self, visible: bool) {
        self.imp().remove_button.set_visible(visible);
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

    pub fn connect_remove_provider<F: Fn(&SwDiscoverySourceRow) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        let obj = self.clone();
        self.connect_local("remove-provider", true, move |_: &[glib::Value]| {
            f(&obj);
            None
        })
    }
}
