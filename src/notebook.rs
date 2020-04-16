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
