use std::collections::VecDeque;
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    movie1: String,
    movie2: String,
    actor: String,
}

pub struct Connection {
    pub to: String,
    pub from: String,
    pub actor: HashSet<String>,
}

pub struct Path {
    pub linkage: Option<Vec<Connection>>,
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

type ConnectionGraph = HashMap<String, HashMap<String, HashSet<String>>>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MovieGraph {
    // Map of movie names, their respective linked movie, and all the actors that connect them
    adj_list: ConnectionGraph,
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

        let now = Instant::now();

        let mut count = 0;

        let total = 14_000_000;
        let mut last_percent = 0;

        for line in iter {
            match line {
                Ok(record) => {
                    count += 1;

                    let percent = count * 100 / total;

                    if percent > last_percent {
                        println!("{}% there", percent);
                        last_percent = percent;
                    }

                    let movie1 = &record.movie1;
                    let movie2 = &record.movie2;
                    let actor = &record.actor;

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

    pub fn bfs_traversal(&self, from: String, to: String) -> Path {
        let start = Instant::now();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Store indices to strings instead of cloning strings
        let mut parent_graph: HashMap<&str, (&str, &HashSet<String>)> = HashMap::new();

        queue.push_back(from.as_str());
        visited.insert(from.as_str());

        while let Some(node) = queue.pop_front() {
            if node == to {
                println!("BFS Found! With time of: {:.2?}", start.elapsed());
                let mut path: Vec<Connection> = Vec::new();
                let mut current_node = node;

                // Pre-allocate the vector with estimated capacity
                path.reserve(parent_graph.len());

                while current_node != from {
                    if let Some(&(parent, actors)) = parent_graph.get(current_node) {
                        path.push(Connection {
                            to: current_node.to_string(),
                            from: parent.to_string(),
                            actor: actors.clone(),
                        });
                        current_node = parent;
                    }
                }

                path.reverse();
                return Path {
                    linkage: Some(path),
                };
            }

            if let Some(neighbors) = self.adj_list.get(node) {
                for (neighbor, actors) in neighbors {
                    if !visited.contains(neighbor.as_str()) {
                        queue.push_back(neighbor.as_str());
                        visited.insert(neighbor.as_str());
                        parent_graph.insert(neighbor.as_str(), (node, actors));
                    }
                }
            }
        }

        println!("No connection. Done in {:.2?}", start.elapsed());
        Path { linkage: None }
    }

    pub fn is_connected(&self, from: String, to: String) -> Option<Connection> {
        if let Some(neighbors) = self.adj_list.get(&from) {
            if neighbors.contains_key(&to) {
                return Some(Connection {
                    to: to.clone(),
                    from,
                    actor: neighbors.get(&to).unwrap().clone(),
                });
            } else {
                None
            }
        } else {
            None
        }
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
