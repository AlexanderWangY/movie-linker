use csv::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    actor: String,
    movies: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path("moviedatanew.csv")?;

    let mut iter = reader.deserialize();

    // get the user's index to check
    print!("Please enter a floating point number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();

    if let Some(result) = iter.nth(num) {
        let record: Record = result?;

        let movies: Vec<String> = serde_json::from_str(&record.movies)?;

        println!("Actor: {}", record.actor);
        println!("Movies: {:?}", movies);
    } else {
        println!("Row {} not found!", num);
    }
    Ok(())
}
