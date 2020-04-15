use serde::{Serialize, Deserialize};
use uuid::Uuid;

extern crate chrono;
use chrono::{NaiveDateTime, naive::serde::ts_seconds};

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    
    pub tags: Option<Vec<String>>,
   
    pub title: Option<String>,
   
    #[serde(with = "ts_seconds")]
    pub updated_at: NaiveDateTime,
   
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
struct Cell {
    #[serde(alias = "type")]
    celltype: String, // Type string can be: code, diagram, latex, markdown, text
    data: String,
}

#[derive(Serialize, Deserialize)]
struct NoteContent {
    title: String,
    cells: Vec<Cell>
}


impl Note {
    pub fn new(s: &str) -> Note {
        return serde_json::from_str(s).unwrap();
    }
}
