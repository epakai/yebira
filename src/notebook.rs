use serde::{Serialize, Deserialize};
use gio::prelude::*;
use gtk::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Notebook {
    pub uuid: String,
    pub name: Option<String>,
    children: Option<Vec<Notebook>>,
}

impl Notebook {
    //pub fn new() -> Notebook {
    //}
    
    pub fn new(s: &str) -> Notebook {
        let n: Notebook = serde_json::from_str(s).unwrap();
        /* TODO can serde, somehow notice name is unpopulated and run
         * to the UUID.qvnotebook/meta.json to fetch it?
         */
        return n;
    }
    
    pub fn to_tree_store(self) -> gtk::TreeStore {
        let tree_store = gtk::TreeStore::new(
            // notebook name, note count
            &[String::static_type(), u32::static_type()]);

        let iter: Option::<gtk::TreeIter> = None;
        fn load_tree_store(ts: &gtk::TreeStore, nb: Notebook,
                           parent: &Option::<gtk::TreeIter>) {
            match nb.children {
                None => {
                    ts.insert_with_values(
                        // TODO  somehow load up the notebook by uuid
                        /*
                         * Also we still need to keep the uuid around
                         * because when we convert back to a Notebook and
                         * serialize the data, we have to sync names to 
                         * individual notebooks, while maintaining the order
                         * in the top level meta.json
                         */
                        parent.as_ref(), None, &[0], &[&nb.uuid]);
                }
                Some(x) => {
                    let new_parent = Some(ts.insert_with_values(
                        parent.as_ref(), None, &[0, 1],
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
