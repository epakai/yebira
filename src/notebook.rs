use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Notebook {
    pub uuid: String,

    // Names are only defined in the individual notebooks
    pub name: Option<String>,
}

impl Notebook {
    //pub fn new() -> Notebook {
    //}
    
    pub fn new(s: &str) -> Notebook {
        return serde_json::from_str(s).unwrap();
    }
}
