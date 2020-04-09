extern crate gio;
extern crate gtk;
extern crate glib;
extern crate pango;
extern crate xdg;

use gio::prelude::*;
use gtk::prelude::*;
use std::env;
use std::ffi::CString;

mod notebook;
use notebook::Notebook;

fn main() {
    let yebira_dirs = xdg::BaseDirectories::with_prefix("yebira").unwrap();
    let data_path = yebira_dirs.create_data_directory(yebira_dirs.get_data_home())
        .expect("Could not create data directory");

    let uiapp = gtk::Application::new(
        Some("org.drislock.yebira"),
        gio::ApplicationFlags::empty(),
    ).expect("Application init failed");
    
    uiapp.connect_activate(|_app| {
        let builder = gtk::Builder::new_from_string(include_str!("yebira.glade"));
        let win: gtk::ApplicationWindow = builder.get_object("YebiraWindow").unwrap();

        win.set_application(Some(_app));

        win.connect_delete_event(move |window, _| {
            window.destroy();
            Inhibit(false)
        });
       
        // Load Notebook List
        let test_meta = include_str!("meta.json");
        let notebook_index: Notebook = Notebook::new(test_meta);

        let notebook_tree = notebook_index.to_tree_store();
        let notebook_treeview: gtk::TreeView = builder.get_object("NotebooksTree").unwrap();

        let name_column = gtk::TreeViewColumn::new();
        let name_cell = gtk::CellRendererText::new();
        name_cell.set_property_ellipsize(pango::EllipsizeMode::End);
        name_cell.set_property_ellipsize_set(true);
        name_column.pack_start(&name_cell, true);
        name_column.add_attribute(&name_cell, "text", 0);
        name_column.set_expand(true);
        notebook_treeview.append_column(&name_column);

        let count_column = gtk::TreeViewColumn::new();
        let count_cell = gtk::CellRendererText::new();
        count_cell.set_alignment(1.0,0.5);
        count_column.pack_start(&count_cell, true);
        count_column.add_attribute(&count_cell, "text", 1);
        /*
        fn count_cell_data_function (tree_view_col: &gtk::TreeViewColumn,
                                     cell_render: &gtk::CellRendererText,
                                     tree_model: &gtk::TreeModel,
                                     iter: &gtk::TreeIter) {
            let c_zero_string = CString::new("0").expect("CString::new failed");
            unsafe {
                let zero_gstring = glib::GString::new(c_zero_string.into_raw()); 
                if tree_model.get_string_from_iter(iter) == Some(zero_gstring) {
                    cell_render.set_property_text(Some(""))
                }
            }
        }
        gtk::TreeViewColumnExt::set_cell_data_func(&count_column, &count_cell,
                                                   Some(count_cell_data_function));
        */
        notebook_treeview.append_column(&count_column);

        notebook_treeview.set_model(Some(&notebook_tree));
        notebook_treeview.set_headers_visible(false);
        // TODO: eventually will will store the last path/note/cell and open to that
        let path = gtk::TreePath::new_from_string("0");
        notebook_treeview.expand_row(&path, false);

        win.show_all();
    });

    uiapp.run(&env::args().collect::<Vec<_>>());
}
