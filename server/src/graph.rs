use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    movie: String,
    actors: String, // Json of a vector of Strings
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MovieGraph {
    // Map of movie names, their respective linked movie, and all the actors that connect them
    adj_list: HashMap<String, HashMap<String, HashSet<String>>>,
}

#[allow(dead_code)]
impl MovieGraph {
    pub fn new() -> Self {
        MovieGraph {
            adj_list: HashMap::new(),
        }
    }

    pub fn parse_csv(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let mut reader = Reader::from_path(path)?;

        let iter = reader.deserialize::<Record>();

        let mut count = 0;

        for line in iter {
            match line {
                Ok(record) => {
                    let movie = &record.movie;
                    let actors: Vec<String> = serde_json::from_str(&record.actors)?;

                    count += 1;
                    println!("{}. {}, {:?}", count, movie, actors);
                }
                Err(err) => println!("Error parsing: {:?}", err),
            }
        }

        Ok(())
    }
}
