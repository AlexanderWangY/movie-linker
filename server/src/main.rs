mod graph;

use std::error::Error;

use graph::MovieGraph;

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph = MovieGraph::new();

    match graph.parse_csv(String::from("output.csv")) {
        Ok(_) => {
            println!("Successfully parsed through")
        }
        Err(err) => {
            println!("Something went wrong {:?}", err)
        }
    }

    graph.bfs_traversal("Beg".to_string(), "Digging Up the Marrow".to_string());
    Ok(())
}
