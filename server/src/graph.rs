use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Movie {
    name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MovieGraph {
    movies: HashMap<String, Movie>,
    // Map of movie names, their respective linked movie, and all the actors that connect them
    edges: HashMap<String, HashMap<String, HashSet<String>>>,
}

#[allow(dead_code)]
impl MovieGraph {
    pub fn new() -> Self {
        MovieGraph {
            movies: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_movie()
}
