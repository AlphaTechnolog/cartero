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
use gtk::glib;

use crate::objects::Collection;

mod imp {
    use std::cell::OnceCell;

    use glib::subclass::InitializingObject;
    use glib::Properties;
    use gtk::subclass::prelude::*;
    use gtk::{prelude::*, CompositeTemplate};

    use crate::objects::Collection;

    #[derive(CompositeTemplate, Default, Properties)]
    #[template(resource = "/es/danirod/Cartero/collection_pane.ui")]
    #[properties(wrapper_type = super::CollectionPane)]
    pub struct CollectionPane {
        #[template_child]
        collection_name: TemplateChild<gtk::Entry>,

        #[property(get, construct_only)]
        collection: OnceCell<Collection>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CollectionPane {
        const NAME: &'static str = "CarteroCollectionPane";
        type Type = super::CollectionPane;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CollectionPane {
        fn constructed(&self) {
            self.parent_constructed();

            if let Some(col) = self.collection.get() {
                self.collection_name.set_text(&col.name());
            }
        }
    }

    impl WidgetImpl for CollectionPane {}

    impl BoxImpl for CollectionPane {}

    #[gtk::template_callbacks]
    impl CollectionPane {}
}

glib::wrapper! {
    pub struct CollectionPane(ObjectSubclass<imp::CollectionPane>)
        @extends gtk::Widget, gtk::Box;
}

impl CollectionPane {
    pub fn new(col: &Collection) -> Self {
        Object::builder().property("collection", col).build()
    }
}
