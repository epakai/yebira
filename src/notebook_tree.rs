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

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use gio::prelude::*;
use gtk::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct NotebookTree {
    // uuid is a String because quiver
    pub uuid: String,

    // Names are only defined in the individual notebooks, but we're
    // going to populate it here for our tree
    #[serde(skip)]
    pub name: Option<String>,
    
    // Children only exist in the root Notebook to provide a tree
    // structure for notebook organization
    children: Option<Vec<NotebookTree>>,
   
    // Only used at runtime for tracking note count for the tree view display
    #[serde(skip)]
    pub note_count: Option<u32>,
}

impl NotebookTree {
    //pub fn new() -> Notebook {
    //}
    
    pub fn new(s: &str) -> NotebookTree {
        return serde_json::from_str(s).unwrap();
        /* TODO can serde, somehow notice name is unpopulated and run
         * to the UUID.qvnotebook/meta.json to fetch it?
         */
    }
    
    // Only used for root meta.json
    pub fn to_tree_store(self) -> gtk::TreeStore {
        let tree_store = gtk::TreeStore::new(
            // notebook name, note count
            &[String::static_type(), u32::static_type()]);

        let iter: Option::<gtk::TreeIter> = None;
        fn load_tree_store(ts: &gtk::TreeStore, nb: NotebookTree,
                           parent: &Option::<gtk::TreeIter>) {
            match nb.children {
                None => {
                    ts.insert_with_values(
                        // TODO  somehow load up the notebook by uuid
                        /*
                         * Also we still need to keep the uuid around
                         * because when we convert back to a NotebookTree and
                         * serialize the data, we have to sync names to 
                         * individual notebooks, while maintaining the order
                         * in the top level meta.json
                         */
                        parent.as_ref(), None, &[0], &[&nb.uuid]);
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
        }
        load_tree_store(&tree_store, self, &iter);
        return tree_store;
    }


}
