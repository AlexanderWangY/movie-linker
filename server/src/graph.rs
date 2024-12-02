use std::{
    collections::{HashMap, HashSet},
    error::Error, hash::Hash,
};

use std::fs::File;
use std::path::Path;

use csv::Reader;
use serde::{Deserialize, Serialize};

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
                    let movie: String = match serde_json::from_str(&record.movie) {
                        Ok(movie) => movie,
                        Err(_) => record.movie.clone().to_string(),
                    };

                    let actors: Vec<String> = match serde_json::from_str(&record.actors) {
                        Ok(actors) => actors,
                        Err(err) => {
                            eprintln!("Failed to deserialize actors: {}", err);
                            vec![]
                        }
                    };
                    // This parses into a string and a vector of strings for actors
                    // There is a weird bug where it'll stop printing at 186,738 lines, this might be because of an unexpected token?

                    count += 1;
                    println!("{}. {}, {}", count, movie, actors[0]);

                    //Insert a vertex after the respective movie has been read
                    self.adj_list.insert(movie.to_string(), HashMap::new());
                }
                Err(err) => println!("Error parsing: {:?}", err),
            }
        }
        Ok(())
    }

    pub fn buildGraph(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let mut current_map : HashMap<String, Vec<String>> = HashMap::new();
        type MovieTitles = Vec<String>;
        let mut actor_list: Vec<String> = Vec::new();
        let mut graph_of_movies: HashMap<String, Vec<String>> = HashMap::new();
        let empty: Vec<String> = Vec::new();
        
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize(){
            let record : MovieTitles = result?;
        }

        Ok(())
    }
}
