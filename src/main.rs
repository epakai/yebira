/* 
 * This file is part of Yebira.
 *
 * Copyright 2020 Joshua Honeycutt
 *
 * Yebira is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Yebira is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with Yebira.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate gio;
extern crate gtk;
extern crate glib;
extern crate pango;
extern crate xdg;

use std::collections::HashMap;
use gio::prelude::*;
use gtk::prelude::*;
use std::env;
use std::ffi::CString;

mod notebook_tree;
use notebook_tree::NotebookTree;
mod notebook;
use notebook::Notebook;
/*
   mod note;
   use note::Note;
   */

/*fn load_notebooks(nb_tree: &gtk::TreeStore,
                  notebooks: &HashMap<String, notebook::Notebook>,
                  parent: &Option::<gtk::TreeIter>) {
    match nb_tree.children {
        None => {
            //
            notebooks.insert(); 
            ts.insert_with_values(parent.as_ref(), None, &[0], &[&nb.uuid]);
        }
        Some(x) => {
            let new_parent = Some(ts.insert_with_values(
                    parent.as_ref(), None, &[0, 1],
                    // TODO len() is the notebook count, but we actually
                    // want to use the total number of notes in this notebook
                    // and it's children
                    &[&nb.uuid, &(x.len() as u32)]));
            for child in x {
                load_tree_store(ts, child, &new_parent);
            }
        }
    }
    load_tree_store(&tree_store, self, &parent);
    return tree_store;
}
*/

fn main() {

    let uiapp = gtk::Application::new(
        Some("org.drislock.yebira"),
        gio::ApplicationFlags::empty(),
        ).expect("Application init failed");

    uiapp.connect_startup(move |app| {
        // In some cases this data may not be byte-aligned resulting in a crash
        // when the builder is created. The array syntax causes alignment, but
        // this is undocumented.
        // https://github.com/gtk-rs/glib/issues/120#issuecomment-250188738
        let resource_data: &[u8] = &include_bytes!("res/resources.gresource")[..];
        let bytes = glib::Bytes::from(&resource_data);
        // more obvious way to load a resource file, not guaranteed aligned
        //let bytes = glib::Bytes::from_static(
        //    include_bytes!("res/resources.gresource"));
        let res = gio::Resource::new_from_data(&bytes).unwrap();
        gio::resources_register(&res);
        let builder = gtk::Builder::new_from_resource("/org/drislock/Yebira/gtk/yebira.ui");
        let win: gtk::ApplicationWindow = builder.get_object("YebiraWindow").unwrap();

        win.set_application(Some(app));

        win.connect_delete_event(move |window, _| {
            window.destroy();
            Inhibit(false)
        });

        // Load NotebookTree in a gtk::TreeStore
        let test_meta = include_str!("meta.json");
        let notebook_index: NotebookTree = NotebookTree::new(test_meta);
        let notebook_tree: gtk::TreeStore = notebook_index.to_tree_store();
        let notebook_treeview: gtk::TreeView = builder.get_object("NotebooksTree").unwrap();

        //let mut notebooks: HashMap<String, notebook::Notebook> = HashMap::new();
        //load_notebooks(&notebook_tree, &notebooks, &None);

        // Name Column
        let name_column = gtk::TreeViewColumn::new();
        let name_cell = gtk::CellRendererText::new();
        name_cell.set_property_ellipsize(pango::EllipsizeMode::End);
        name_cell.set_property_ellipsize_set(true);
        name_column.pack_start(&name_cell, true);
        name_column.add_attribute(&name_cell, "text", 0);
        name_column.set_expand(true);
        notebook_treeview.append_column(&name_column);

        // Count Column, Hide count on the root item
        let count_column = gtk::TreeViewColumn::new();
        let count_cell = gtk::CellRendererText::new();
        count_cell.set_alignment(1.0,0.5);
        fn count_cell_data_function (_tree_view_col: &gtk::TreeViewColumn,
                                     cell_render: &gtk::CellRenderer,
                                     tree_model: &gtk::TreeModel,
                                     iter: &gtk::TreeIter) {
            let c_zero_string = CString::new("0").expect("CString::new failed");
            unsafe {
                let zero_gstring = glib::GString::new(c_zero_string.into_raw()); 
                if tree_model.get_string_from_iter(iter) == Some(zero_gstring) {
                    //gtk::CellRendererText::set_property_text(
                    //    gtk::CellRendererText::From(cell_render), Some(""))
                    glib::object::ObjectExt::set_property(cell_render, "text",
                                                          &String::from("")).
                        expect("Failed to set text property on CellRenderer");
                }
            }
        }
        count_column.pack_start(&count_cell, true);
        count_column.add_attribute(&count_cell, "text", 1);
        gtk::TreeViewColumnExt::set_cell_data_func(
            &count_column, &count_cell,
            Some(Box::new(count_cell_data_function)));
        notebook_treeview.append_column(&count_column);

        notebook_treeview.set_model(Some(&notebook_tree));
        notebook_treeview.set_headers_visible(false);
        // TODO: eventually we will store the last path/note/cell and open to that
        let path = gtk::TreePath::new_from_string("0");
        notebook_treeview.expand_row(&path, false);

        app.connect_activate(move |_| {
            win.show_all();
            win.present();
        });
    });

    uiapp.run(&env::args().collect::<Vec<_>>());
}
