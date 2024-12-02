use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    actor: String,
    movies: String,
}

#[allow(dead_code)]
pub struct MovieParser {}

#[allow(dead_code, unused_variables)]
impl MovieParser {
    pub fn parse_csv(path: String) -> Result<HashMap<String, HashSet<String>>, Box<dyn Error>> {
        let mut reader = Reader::from_path(path)?;

        let mut result = HashMap::new();

        for entry in reader.records() {
            let new_record: Record = entry
        }

        Ok(result)
    }
}
