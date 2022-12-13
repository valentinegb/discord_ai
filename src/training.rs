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
fn add_entry(entry: TrainingDataEntry, file_path: &str) -> Result<(), std::io::Error> {
    // Read file contents
    let file_contents = read_to_string(file_path)?;
    // Deserialize file contents
    let mut training_data: Vec<TrainingDataEntry> = from_str(&file_contents)?;

    // Add entry to training data
    training_data.push(entry);

    // Serialize training data
    let serialized_training_data = to_string_pretty(&training_data)?;

    // Write serializde training data to file
    write(file_path, serialized_training_data)?;

    // Function completed successfully, return Ok
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn add_entry_test() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(FILE_PATH);

        write(&file_path, "[]").unwrap();

        add_entry(
            TrainingDataEntry {
                prompt: "Template: Hello?\nResponse: ",
                completions: " Hello.\n",
            },
            file_path.to_str().unwrap(),
        ).unwrap();

        assert_eq!(
            read_to_string(file_path).unwrap(),
"[
  {
    \"prompt\": \"Template: Hello?\\nResponse: \",
    \"completions\": \" Hello.\\n\"
  }
]",
        );
    }
}
