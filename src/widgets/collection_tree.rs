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

use glib::{object::Cast, subclass::types::ObjectSubclassIsExt, Object};
use gtk::TreeListModel;

use crate::objects::Collection;

mod imp {
    use std::cell::{OnceCell, RefCell};
    use std::ops::Deref;

    use adw::subclass::bin::BinImpl;
    use glib::subclass::InitializingObject;
    use glib::{GString, Object};
    use gtk::gio::{ListModel, ListStore};
    use gtk::subclass::prelude::*;
    use gtk::{
        prelude::*, CompositeTemplate, Label, ListItem, ListView, SignalListItemFactory,
        SingleSelection, TreeExpander, TreeListModel, TreeListRow,
    };

    use crate::objects::{Collection, KeyValueItem};
    use crate::widgets::CollectionPane;
    use crate::win::CarteroWindow;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/es/danirod/Cartero/collection_tree.ui")]
    pub struct CollectionTree {
        #[template_child]
        pub(super) selection_model: TemplateChild<SingleSelection>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CollectionTree {
        const NAME: &'static str = "CarteroCollectionTree";
        type Type = super::CollectionTree;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CollectionTree {
        fn constructed(&self) {
            self.parent_constructed();

            let tree_model = self.init_tree_model();
            self.selection_model.set_model(Some(&tree_model));
        }
    }

    impl WidgetImpl for CollectionTree {}

    impl BinImpl for CollectionTree {}

    #[gtk::template_callbacks]
    impl CollectionTree {
        fn init_tree_model(&self) -> TreeListModel {
            let root_model: ListStore = Object::builder()
                .property("item-type", Collection::static_type())
                .build();
            TreeListModel::new(root_model, false, false, |obj: &Object| {
                let is_root = obj.is::<Collection>();
                if is_root {
                    let children: ListStore = Object::builder()
                        .property("item-type", KeyValueItem::static_type())
                        .build();
                    let item = KeyValueItem::default();
                    item.set_header_name("hola");
                    item.set_header_value("hola");
                    children.append(&item);
                    let model = children.upcast::<ListModel>();
                    Some(model)
                } else {
                    None
                }
            })
        }

        pub(super) fn root_model(&self) -> Option<ListStore> {
            self.selection_model
                .model()
                .and_downcast::<TreeListModel>()
                .and_then(|tlm: TreeListModel| Some(tlm.model()))
                .and_downcast::<ListStore>()
        }

        #[template_callback]
        fn on_activate(list: ListView, pos: u32, data: &Object) {
            println!("activate()");
            println!("list = {:?} \n pos = {:?} \n data = {:?}", list, pos, data);

            let model = list.model().unwrap();
            if let Some(item) = model.item(pos) {
                let row = item.downcast::<TreeListRow>().unwrap();
                let item = row.item().unwrap();

                let root = list.root().unwrap();
                let window = root.downcast::<CarteroWindow>().unwrap();

                let the_type = item.type_();
                if the_type == Collection::static_type() {
                    let collection = item.downcast::<Collection>().unwrap();
                    window.open_collection_pane(&collection);
                } else if the_type == KeyValueItem::static_type() {
                    let key_value = item.downcast::<KeyValueItem>().unwrap();
                    println!("Es un item");
                    println!("Es el {:?}", key_value.header_name());
                }
            }
        }

        #[template_callback]
        fn on_factory_setup(_: SignalListItemFactory, obj: &Object) {
            let item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let label = Label::new(Some(""));
            let expander = TreeExpander::new();
            expander.set_child(Some(&label));
            item.set_child(Some(&expander));
        }

        #[template_callback]
        fn on_factory_bind(_: SignalListItemFactory, obj: &Object) {
            let item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let expander = item.child().and_downcast::<TreeExpander>().unwrap();
            let widget = expander.child().and_downcast::<Label>().unwrap();
            let row = item.item().and_downcast::<gtk::TreeListRow>().unwrap();

            expander.set_list_row(Some(&row));

            let gobject = row.item().unwrap();
            if gobject.is::<Collection>() {
                let item = row.item().and_downcast::<Collection>().unwrap();
                widget.set_label(&item.name());
            }

            if gobject.is::<KeyValueItem>() {
                let item = row.item().and_downcast::<KeyValueItem>().unwrap();
                widget.set_label(&item.header_name());
            }
        }

        #[template_callback]
        fn on_factory_unbind(_: SignalListItemFactory, obj: &Object) {
            let item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            let expander = item.child().and_downcast::<TreeExpander>().unwrap();
            let widget = expander.child().and_downcast::<Label>().unwrap();
            expander.set_list_row(None);
            widget.set_label("");
        }

        #[template_callback]
        fn on_factory_teardown(_: SignalListItemFactory, obj: &Object) {
            let item = obj.downcast_ref::<gtk::ListItem>().unwrap();
            item.set_child(Option::<&gtk::Widget>::None);
        }
    }
}

glib::wrapper! {
    pub struct CollectionTree(ObjectSubclass<imp::CollectionTree>)
        @extends gtk::Widget, adw::Bin;
}

impl Default for CollectionTree {
    fn default() -> Self {
        Object::builder().build()
    }
}

impl CollectionTree {
    pub fn insert_collections(&self, cols: &[Collection]) {
        for col in cols {
            self.append_collection(col);
        }
    }

    pub fn append_collection(&self, col: &Collection) {
        let imp = self.imp();

        if let Some(root_model) = imp.root_model() {
            root_model.append(col);
        }
    }
}
