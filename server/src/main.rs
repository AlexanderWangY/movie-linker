mod graph;

use std::error::Error;

use graph::MovieGraph;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut reader = csv::Reader::from_path("moviedatanew.csv")?;
    // for (i, result) in reader.records().enumerate() {
    //     match result {
    //         Ok(_) => {
    //             println!("{i}, processed fine");
    //         }
    //         Err(err) => {
    //             eprintln!("Error on record {}: {}", i, err);
    //             break; // or continue if you want to skip errors
    //         }
    //     }
    // }

    // Ok(())

    let mut graph = MovieGraph::new();

    match graph.parse_csv(String::from("output.csv")) {
        Ok(_) => {
            println!("Successfully parsed through")
        }
        Err(err) => {
            println!("Something went wrong {:?}", err)
        }
    }

    Ok(())
}
