// src/random_quote.rs

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs::File;
use std::io::Read; // Correct crate for YAML

#[derive(Deserialize, Debug, Clone)] // Add `Clone` if you need to clone the `Quote` later
pub struct Quote {
    pub author: String,
    pub quote: String,
}

impl Quote {
    /* pub fn to_string() -> String {
        "Hello World".to_string()
    } */
    // Method to convert Quote to a string
    pub fn to_string(&self) -> String {
        format!("\"{}\" - {}", self.quote, self.author)
    }
}

pub fn get_random_quote() -> Result<Quote, String> {
    // Open the file
    let path = "/home/x/projects/rust/quitabitui/quotes/quotes.yaml";
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Deserialize the YAML string into a Vec<Quote>
    let quotes: Vec<Quote> =
        serde_yml::from_str(&contents).map_err(|e| format!("Failed to deserialize YAML: {}", e))?;

    // Check if the vector is empty
    if quotes.is_empty() {
        return Err("Quotes' List Is Empty".to_string());
    }

    // Generate a random index and select a random quote
    let mut rng = thread_rng();
    quotes
        .choose(&mut rng)
        .cloned()
        .ok_or_else(|| "Failed to select a random quote.".to_string())
}
