// Copyright 2024 the Cartero authors
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
//
// SPDX-License-Identifier: GPL-3.0-or-later

use glib::Object;
use gtk::gio::{self, ListModel, ListStore, Settings};

use crate::config::APP_ID;
use crate::objects::Collection;
use crate::win::CarteroWindow;

mod imp {
    use std::cell::OnceCell;

    use adw::prelude::*;
    use adw::subclass::application::AdwApplicationImpl;
    use gio::ListStore;
    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use glib::Properties;
    use gtk::gio::Settings;
    use gtk::subclass::prelude::*;
    use gtk::subclass::{application::GtkApplicationImpl, prelude::ApplicationImpl};

    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::CarteroApplication)]
    pub struct CarteroApplication {
        #[property(get, construct_only)]
        pub(super) settings: OnceCell<Settings>,

        #[property(get, construct_only)]
        pub(super) collections: OnceCell<ListStore>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CarteroApplication {
        const NAME: &'static str = "CarteroApplication";
        type Type = super::CarteroApplication;
        type ParentType = adw::Application;
    }

    #[glib::derived_properties]
    impl ObjectImpl for CarteroApplication {}

    impl ApplicationImpl for CarteroApplication {
        fn activate(&self) {
            self.parent_activate();
            self.obj().get_window().present();
        }

        fn startup(&self) {
            self.parent_startup();
            gtk::Window::set_default_icon_name(APP_ID);

            let obj = self.obj();
            obj.set_accels_for_action("win.request", &["<Primary>Return"]);
        }
    }

    impl GtkApplicationImpl for CarteroApplication {}

    impl AdwApplicationImpl for CarteroApplication {}
}

glib::wrapper! {
    pub struct CarteroApplication(ObjectSubclass<imp::CarteroApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;

}

impl Default for CarteroApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl CarteroApplication {
    pub fn new() -> Self {
        let store = ListStore::new::<Collection>();
        let collection = Collection::new_with_title("httpbin.org");
        let collection2 = Collection::new_with_title("pokeapi");
        let collection3 = Collection::new_with_title("random-d.uk");
        store.extend_from_slice(&[collection, collection2, collection3]);

        let settings = Settings::new(APP_ID);
        Object::builder()
            .property("application-id", APP_ID)
            .property("settings", settings)
            .property("collections", store)
            .build()
    }

    pub fn get_window(&self) -> CarteroWindow {
        let win = CarteroWindow::new(self);
        win.assign_settings(&self.settings());
        win
    }
}
