// preferences.rs
//
// Copyright 2021 Martin Pobaschnig
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
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

use adw::prelude::*;
use gettextrs::gettext;
use gtk::gio;
use gtk::glib;
use gtk::glib::clone;
use gtk::glib::GString;
use gtk::subclass::prelude::*;

use crate::GlobalConfigManager;
use crate::VApplication;

mod imp {
    use adw::prelude::*;
    use gtk::{glib, subclass::prelude::*, CompositeTemplate};

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/io/github/mpobaschnig/Vaults/preferences.ui")]
    pub struct VaultSettingsDialog {
        #[template_child]
        pub encrypted_data_directory_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub encrypted_data_directory_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub encrypted_data_directory_error_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub mount_directory_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub mount_directory_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub mount_directory_error_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub general_apply_changes_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VaultSettingsDialog {
        const NAME: &'static str = "VaultSettingsDialog";
        type ParentType = gtk::Dialog;
        type Type = super::PreferencesWindow;

        fn new() -> Self {
            Self {
                general_apply_changes_button: TemplateChild::default(),
                encrypted_data_directory_entry: TemplateChild::default(),
                encrypted_data_directory_button: TemplateChild::default(),
                encrypted_data_directory_error_label: TemplateChild::default(),
                mount_directory_entry: TemplateChild::default(),
                mount_directory_button: TemplateChild::default(),
                mount_directory_error_label: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VaultSettingsDialog {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for VaultSettingsDialog {}
    impl WindowImpl for VaultSettingsDialog {}
    impl DialogImpl for VaultSettingsDialog {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::VaultSettingsDialog>)
        @extends gtk::Widget, gtk::Window, gtk::Dialog;
}

impl PreferencesWindow {
    pub fn new() -> Self {
        let o: Self = glib::Object::new(&[("use-header-bar", &1)])
            .expect("Failed to create PreferencesWindow");

        let window = gio::Application::default()
            .unwrap()
            .downcast_ref::<VApplication>()
            .unwrap()
            .active_window()
            .unwrap();
        o.set_transient_for(Some(&window));

        o.init();

        o.setup_signals();

        o
    }

    fn init(&self) {
        let self_ = imp::VaultSettingsDialog::from_instance(self);

        let global_config = GlobalConfigManager::instance().get_global_config();

        self_
            .encrypted_data_directory_entry
            .set_text(&global_config.encrypted_data_directory.borrow());

        self_
            .mount_directory_entry
            .set_text(&global_config.mount_directory.borrow());
    }

    fn setup_signals(&self) {
        let self_ = imp::VaultSettingsDialog::from_instance(self);

        self_.encrypted_data_directory_entry.connect_text_notify(
            clone!(@weak self as obj => move |_| {
                obj.check_apply_changes_button_enable_conditions();
            }),
        );

        self_.encrypted_data_directory_button.connect_clicked(
            clone!(@weak self as obj => move |_| {
                obj.encrypted_data_directory_button_clicked();
            }),
        );

        self_
            .mount_directory_entry
            .connect_text_notify(clone!(@weak self as obj => move |_| {
                obj.check_apply_changes_button_enable_conditions();
            }));

        self_
            .mount_directory_button
            .connect_clicked(clone!(@weak self as obj => move |_| {
                obj.mount_directory_button_clicked();
            }));

        self_
            .general_apply_changes_button
            .connect_clicked(clone!(@weak self as obj => move |_| {
                obj.general_apply_changes_button_clicked();
            }));
    }

    fn encrypted_data_directory_button_clicked(&self) {
        let dialog = gtk::FileChooserDialog::new(
            Some(&gettext("Choose Encrypted Data Directory")),
            Some(self),
            gtk::FileChooserAction::SelectFolder,
            &[
                (&gettext("Cancel"), gtk::ResponseType::Cancel),
                (&gettext("Select"), gtk::ResponseType::Accept),
            ],
        );

        dialog.set_transient_for(Some(self));

        dialog.connect_response(clone!(@weak self as obj => move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                let file = dialog.file().unwrap();
                let path = String::from(file.path().unwrap().as_os_str().to_str().unwrap());
                let self_ = imp::VaultSettingsDialog::from_instance(&obj);
                self_.encrypted_data_directory_entry.set_text(&path);
            }

            dialog.destroy();
        }));

        dialog.show();
    }

    fn mount_directory_button_clicked(&self) {
        let dialog = gtk::FileChooserDialog::new(
            Some(&gettext("Choose Mount Directory")),
            Some(self),
            gtk::FileChooserAction::SelectFolder,
            &[
                (&gettext("Cancel"), gtk::ResponseType::Cancel),
                (&gettext("Select"), gtk::ResponseType::Accept),
            ],
        );

        dialog.set_transient_for(Some(self));

        dialog.connect_response(clone!(@weak self as obj => move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                let file = dialog.file().unwrap();
                let path = String::from(file.path().unwrap().as_os_str().to_str().unwrap());
                let self_ = imp::VaultSettingsDialog::from_instance(&obj);
                self_.mount_directory_entry.set_text(&path);
            }

            dialog.destroy();
        }));

        dialog.show();
    }

    fn general_apply_changes_button_clicked(&self) {
        let self_ = imp::VaultSettingsDialog::from_instance(self);

        let encrypted_data_directory = self_.encrypted_data_directory_entry.text().to_string();
        let mount_directory = self_.mount_directory_entry.text().to_string();

        GlobalConfigManager::instance().set_encrypted_data_directory(encrypted_data_directory);
        GlobalConfigManager::instance().set_mount_directory(mount_directory);

        GlobalConfigManager::instance().write_config();
    }

    fn are_directories_different(
        &self,
        encrypted_data_directory: &GString,
        mount_directory: &GString,
    ) -> bool {
        let self_ = imp::VaultSettingsDialog::from_instance(self);

        if encrypted_data_directory.eq(mount_directory) {
            self_
                .mount_directory_error_label
                .set_text(&gettext("Directories must not be equal."));
            self_.mount_directory_error_label.set_visible(true);

            false
        } else {
            self_.mount_directory_error_label.set_visible(false);

            true
        }
    }

    fn check_apply_changes_button_enable_conditions(&self) {
        let self_ = imp::VaultSettingsDialog::from_instance(self);

        let encrypted_data_directory = self_.encrypted_data_directory_entry.text();
        let mount_directory = self_.mount_directory_entry.text();

        let are_directories_different =
            self.are_directories_different(&encrypted_data_directory, &mount_directory);

        if are_directories_different {
            self_.general_apply_changes_button.set_sensitive(true);
        } else {
            self_.general_apply_changes_button.set_sensitive(false);
        }
    }
}
