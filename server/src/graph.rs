use csv::Reader;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    option::Option,
};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    movie1: String,
    movie2: String,
    actor: String,
}

#[derive(Serialize)]
pub struct Connection {
    pub to: String,
    pub from: String,
    pub actor: HashSet<String>,
}

pub struct Path {
    pub linkage: Option<Vec<Connection>>,
    pub time_elapsed: u128,  // milliseconds
    pub traverse_count: u32, // Count of nodes traversed
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

#[allow(clippy::comparison_chain)]
fn update_progress(count: u64, total: u64, last_percent: &mut u8) -> io::Result<()> {
    const UPDATE_FREQUENCY: u8 = 5; // Only update every 5%

    let percent = ((count as f64 / total as f64) * 100.0) as u8;
    if percent > *last_percent && percent % UPDATE_FREQUENCY == 0 {
        print!("\r[");
        let bars = percent / 2;
        for i in 0..50 {
            if i < bars {
                print!("=");
            } else if i == bars {
                print!(">");
            } else {
                print!(" ");
            }
        }
        print!("] {}%", percent);
        io::stdout().flush()?;
        *last_percent = percent;
    }
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MovieGraph {
    // Map of movie names, their respective linked movie, and all the actors that connect them
    pub adj_list: ConnectionGraph,
}

#[allow(dead_code)]
impl MovieGraph {
    pub fn new() -> Self {
        MovieGraph {
            adj_list: HashMap::new(),
        }
    }

    #[allow(unused_must_use)]
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

                    update_progress(count, total, &mut last_percent);

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

        println!();
        println!("Time taken: {:.2?}", now.elapsed());

        Ok(())
    }

    pub fn bfs_traversal(&self, from: String, to: String) -> Path {
        let mut visited_count: u32 = 1; // One because first node
        let start = Instant::now();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Store indices to strings instead of cloning strings
        let mut parent_graph: HashMap<&str, (&str, &HashSet<String>)> = HashMap::new();

        queue.push_back(from.as_str());
        visited.insert(from.as_str());

        while let Some(node) = queue.pop_front() {
            visited_count += 1;

            if node == to {
                let elapsed = start.elapsed();
                println!("BFS Found! With time of: {:.2?}", elapsed);
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
                    time_elapsed: elapsed.as_millis(),
                    traverse_count: visited_count,
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

        let elapsed = start.elapsed();
        println!("No connection. Done in {:.2?}", elapsed);
        Path {
            linkage: None,
            time_elapsed: elapsed.as_millis(),
            traverse_count: visited_count,
        }
    }

    pub fn newbfs(&self, src: String, dest: String) -> Path {
        let mut visited_count: u32 = 0;
        let now = Instant::now();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut connections: Vec<Connection> = Vec::new();

        visited.insert(src.clone());
        queue.push_back(src.clone());

        while !queue.is_empty() {
            let front = queue.pop_front();

            visited_count += 1;

            for i in &self.adj_list[&front.clone().unwrap()] {
                if *i.0.to_string() == dest {
                    let elapsed = now.elapsed();
                    println!("Time taken using BFS: {:.2?}", elapsed);
                    return Path {
                        linkage: Some(connections),
                        time_elapsed: elapsed.as_millis(),
                        traverse_count: visited_count,
                    };
                }

                if !visited.contains(i.0) {
                    visited.insert(i.0.to_string());
                    queue.push_back(i.0.to_string());
                    connections.push(Connection {
                        from: front.clone().unwrap(),
                        to: i.0.clone(),
                        actor: i.1.clone(),
                    });
                }
            }
        }
        let elapsed = now.elapsed();
        println!(
            "No connections between these movies! Done in: {:.2?}",
            elapsed
        );

        Path {
            linkage: None,
            time_elapsed: elapsed.as_millis(),
            traverse_count: visited_count,
        }
    }

    pub fn newdfs(&self, src: String, dest: String) -> Path {
        let mut visited_count: u32 = 0;
        let now = Instant::now();
        let mut visited = HashSet::new();
        let mut stack = VecDeque::new();
        let mut connections: Vec<Connection> = Vec::new();

        visited.insert(src.clone());
        stack.push_front(src.clone());

        while !stack.is_empty() {
            let front = stack.pop_front();

            visited_count += 1;

            for i in &self.adj_list[&front.clone().unwrap()] {
                if *i.0.to_string() == dest {
                    let elapsed = now.elapsed();

                    println!("Time taken using DFS: {:.2?}", elapsed);
                    return Path {
                        linkage: Some(connections),
                        time_elapsed: elapsed.as_millis(),
                        traverse_count: visited_count,
                    };
                }

                if !visited.contains(i.0) {
                    visited.insert(i.0.to_string());
                    stack.push_front(i.0.to_string());
                    connections.push(Connection {
                        from: front.clone().unwrap(),
                        to: i.0.clone(),
                        actor: i.1.clone(),
                    });
                }
            }
        }
        let elapsed = now.elapsed();
        println!(
            "No connections between these movies! Done in: {:.2?}",
            elapsed
        );

        Path {
            linkage: None,
            time_elapsed: elapsed.as_millis(),
            traverse_count: visited_count,
        }
    }

    //DFS Traversal: First time working with multithreading
    pub fn dfs_traversal(&self, from: String, to: String) -> Path {
        let mut visited_count: u32 = 0;

        let start = Instant::now(); //Start stopwatch
                                    //Initialize necessary values: visited, "stack", overall graph
                                    //Graph is adjacency list of [Movie] => [Linked Movie, Actors that link them]
        let mut visited = HashSet::new();
        let mut stack = VecDeque::new();

        let mut parent_graph: HashMap<&str, (&str, &HashSet<String>)> = HashMap::new();

        //Push source into visited and onto stack like normal
        visited.insert(from.as_str());
        stack.push_front(from.as_str());

        //While getting arbitary vertex (the one at the top of the stack)
        while let Some(vertex) = stack.pop_front() {
            visited_count += 1;

            //If vertices are equal, stop the clock...
            if vertex == to {
                //And write the path consisting of every connection made so far
                let mut path: Vec<Connection> = Vec::new();
                let mut v = vertex;

                //While graph has not returned to the start vertex:
                while v != from {
                    if let Some(&(parent, actors)) = parent_graph.get(v) {
                        //Trace is each individual connection made
                        let trace = Connection {
                            from: v.to_string(),
                            to: parent.to_string(),
                            actor: actors.clone(),
                        };

                        path.push(trace);
                        v = parent;
                    }
                }

                //Reverse the process as this is when DFS is being "launched" back
                path.reverse();

                let elapsed = start.elapsed();
                println!("DFS Path found! Time taken: {:.2?}", elapsed);

                //Return with appropriate path
                return Path {
                    linkage: Some(path),
                    time_elapsed: elapsed.as_millis(),
                    traverse_count: visited_count,
                };
            }

            //If they are not equal, then get every neighbor and do DFS as normal
            //Insert onto 'stack', visited list, and insert into overall graph.
            //Final path will be returned when path is found
            if let Some(neighbors) = self.adj_list.get(vertex) {
                for (neighbor, connected_actors) in neighbors {
                    if !visited.contains(neighbor.as_str()) {
                        stack.push_front(neighbor.as_str());
                        visited.insert(neighbor.as_str());
                        parent_graph.insert(neighbor.as_str(), (vertex, connected_actors));
                    }
                }
            }
        }

        //If none of the loops run, then there are is no S-T path- return an empty path (initialized at start)
        let elapsed = start.elapsed();
        println!(
            "No connection between these movies! Done in: {:.2?}",
            elapsed
        );

        Path {
            linkage: None,
            time_elapsed: elapsed.as_millis(),
            traverse_count: visited_count,
        }
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

    pub fn are_neighbors(&self, from: String, to: String) -> bool {
        if self.adj_list[&from].contains_key(&to) {
            return true;
        }

        false
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
// #[cfg(test)]
//     mod tests{
//         use super::*;

//         #[test]
//         fn test_dfs(){
//             println!("Test running!");
//             let mut movies = MovieGraph{adj_list:HashMap::new()};
//             let _ = movies.parse_csv(String::from("src/output.csv"));
//             println!("{:?}", movies.adj_list["Beg"]["Digging Up the Marrow"]);
//             let x = movies.dfs_traversal("Beg".to_string(), "Digging Up the Marrow".to_string());
//             let mut e = "".to_string();

//             while let Some(ref i) = x.linkage{
//                 for j in i{
//                     println!("{}", j.to);
//                     e += &j.to;
//                     e += " ";
//                     e += &j.from;
//                 }
//             }

//             println!("{}", e);

//         }
//     }
