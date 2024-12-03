#[allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    error::Error, hash::Hash,
};

use std::{fs::File, io::{BufWriter, Write}};
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
    adj_list: HashMap<String, Vec<String>>,
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
        let mut current_map: HashMap<String, Vec<String>> = HashMap::new();
        let iter = reader.deserialize::<Record>();

        let graphfile = File::create("graphfile")?;
        let mut writer = BufWriter::new(graphfile);
        let mut count = 0;

        println!("Going through values...");
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
                    println!("{}", count);
                    
                    //JSON Script- be sure to credit pages at the end
                    current_map.insert(movie, actors);
                    
                }
                Err(err) => println!("Error parsing: {:?}", err),
            }
        }

        println!("Creating graph file...");
        let mut films: Vec<String> = Vec::new();
        for (first_key, first_val) in &current_map{
            let _ = serde_json::to_writer(&mut writer, &first_key);
            for(second_key, second_val) in &current_map{
                for i in second_val.clone(){
                    if first_val.contains(&i) && first_val != second_val && !films.contains(second_key){
                        films.push(second_key.to_string());
                    }
                }

                if films.len() > 0{
                    let _ = serde_json::to_writer(&mut writer, &films);
                    films.clear();
                }
            }
            writer.write( b"\n")?;
            let _ = writer.flush();
        }
        

        Ok(())
    }
}
