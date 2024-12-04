#[allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    error::Error, hash::Hash,
};

use std::{collections::VecDeque, fs::File, io::{BufWriter, Write}, time::Instant};
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    movie: String,
    actors: String, // Json of a vector of Strings
}

#[derive(Debug)]
struct Connection{
    actors: Vec<String>, 
    from: String,
    to: String
}

//Hopefully this is allowed???
#[allow(unconditional_recursion)]
impl Connection {
    pub fn clone(&mut self) -> Connection{
        let x = self.clone();
        return x;
    }
}

#[derive(Debug)]
struct Path{
    path: Vec<Connection>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MovieGraph {
    // Map of movie names, their respective linked movie, and all the actors that connect them
    adj_list: HashMap<String, HashMap<String, HashSet<String>>>,
    final_value: Connection,
    path: Path
}

#[allow(dead_code)]
impl MovieGraph {
    pub fn new() -> Self {
        MovieGraph {
            adj_list: HashMap::new(),
            final_value: Connection { 
                from: ("".to_string()), 
                to: ("".to_string()), 
                actors: (Vec::<String>::new())
            },
            path: Path{
                path: vec![]
            }
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

    fn bfs_traversal(&mut self, src: String, des: String) -> (){
        self.final_value.from = src.clone();
        self.final_value.to = des.clone();

        let start = Instant::now();
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut deq: VecDeque<String> = Default::default();
        
        visited.insert(src.clone(), true);
        deq.push_back(src);

        while !deq.is_empty(){
            self.final_value.actors.push(deq.front().expect("").to_string());
            deq.pop_front();

            for i in &self.adj_list{
                if !visited[i.0]{
                    visited.insert(i.0.to_string(), true);
                    deq.push_back(i.0.to_string());
                }
            }
        }

        let timelapsed = Instant::now();
        println!("Time Elapsed using BFS: {:?}", timelapsed.duration_since(start));
        
        self.path.path.push(self.final_value.clone());
    }

    //Nice little change of code :D
    fn dfs_traversal(&mut self, src: String, des: String) -> (){
        self.final_value.from = src.clone();
        self.final_value.to = des.clone();

        let start = Instant::now();
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut deq: VecDeque<String> = Default::default();

        while !deq.is_empty(){
            self.final_value.actors.push(deq.front().expect("").to_string());
            deq.pop_front();

            for i in &self.adj_list{
                if !visited[i.0]{
                    visited.insert(i.0.to_string(), true);
                    deq.push_front(i.0.to_string());
                }
            }
        }

        let timelapsed = Instant::now();
        println!("Time Elapsed using DFS{:?}", timelapsed.duration_since(start));
        self.path.path.push(self.final_value.clone());
    }
}
