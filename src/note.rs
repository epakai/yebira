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
