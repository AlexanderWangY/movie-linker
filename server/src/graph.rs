use std::time::Instant;
#[allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    hash::Hash,
};

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    movie1: String,
    movie2: String,
    actor: String,
}

// #[derive(Debug)]
// struct Connection{
//     actors: Vec<String>,
//     from: String,
//     to: String
// }

// //Hopefully this is allowed???
// #[allow(unconditional_recursion)]
// impl Connection {
//     pub fn clone(&self) -> Self{
//         return self.clone();
//     }
// }

// #[derive(Debug)]
// struct Path{
//     path: Vec<Connection>
// }

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

        let now = Instant::now();
        for line in iter {
            match line {
                Ok(record) => {
                    count += 1;

                    let movie1 = &record.movie1;
                    let movie2 = &record.movie2;
                    let actor = &record.actor;

                    println!("{count} From: {movie1} <-> To: {movie2} Actor: {actor}");

                    self.adj_list
                        .entry(movie1.clone())
                        .or_default()
                        .entry(movie2.clone())
                        .or_default()
                        .insert(actor.clone());

                    self.adj_list
                        .entry(movie2.clone())
                        .or_default()
                        .entry(movie1.clone())
                        .or_default()
                        .insert(actor.clone());
                }
                Err(err) => println!("Error parsing: {:?}", err),
            }
        }

        println!("Time taken: {:.2?}", now.elapsed());

        Ok(())
    }

    // fn bfs_traversal(&mut self, src: String, des: String) -> () {
    //     self.final_value.from = src.clone();
    //     self.final_value.to = des.clone();

    //     let start = Instant::now();
    //     let mut visited: HashMap<String, bool> = HashMap::new();
    //     let mut deq: VecDeque<String> = Default::default();

    //     visited.insert(src.clone(), true);
    //     deq.push_back(src);

    //     while !deq.is_empty() {
    //         self.final_value
    //             .actors
    //             .push(deq.front().expect("").to_string());
    //         deq.pop_front();

    //         for i in &self.adj_list {
    //             if !visited[i.0] {
    //                 visited.insert(i.0.to_string(), true);
    //                 deq.push_back(i.0.to_string());
    //             }
    //         }
    //     }

    //     let timelapsed = Instant::now();
    //     println!(
    //         "Time Elapsed using BFS: {:?}",
    //         timelapsed.duration_since(start)
    //     );

    //     self.path.path.push(self.final_value.clone());
    // }

    // //Nice little change of code :D
    // fn dfs_traversal(&mut self, src: String, des: String) -> () {
    //     self.final_value.from = src.clone();
    //     self.final_value.to = des.clone();

    //     let start = Instant::now();
    //     let mut visited: HashMap<String, bool> = HashMap::new();
    //     let mut deq: VecDeque<String> = Default::default();

    //     while !deq.is_empty() {
    //         self.final_value
    //             .actors
    //             .push(deq.front().expect("").to_string());
    //         deq.pop_front();

    //         for i in &self.adj_list {
    //             if !visited[i.0] {
    //                 visited.insert(i.0.to_string(), true);
    //                 deq.push_front(i.0.to_string());
    //             }
    //         }
    //     }

    //     let timelapsed = Instant::now();
    //     println!(
    //         "Time Elapsed using DFS{:?}",
    //         timelapsed.duration_since(start)
    //     );
    //     self.path.path.push(self.final_value.clone());
    // }

    //Create getter function for path :)
}
