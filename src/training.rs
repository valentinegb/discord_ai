//! Functions for collecting training data.

use serde::{ Deserialize, Serialize };
use std::fs::{ read_to_string, write };
use serde_json::{ from_str, to_string_pretty };

const FILE_PATH: &str = "training_data.json";

#[derive(Deserialize, Serialize)]
struct TrainingDataEntry<'a> {
    prompt: &'a str,
    completions: &'a str,
}

/// Adds a new entry to the training data JSON file.
/// # Errors
/// This function can return an error if it fails to read or write to the training data JSON file
/// or if it fails to deserialize or serialize.
fn add_entry(entry: TrainingDataEntry) -> Result<(), std::io::Error> {
    // Read file contents
    let file_contents = read_to_string(FILE_PATH)?;
    // Deserialize file contents
    let mut training_data: Vec<TrainingDataEntry> = from_str(&file_contents)?;

    // Add entry to training data
    training_data.push(entry);

    // Serialize training data
    let serialized_training_data = to_string_pretty(&training_data)?;

    // Write serializde training data to file
    write(FILE_PATH, serialized_training_data)?;

    // Function completed successfully, return Ok
    Ok(())
}
