#[macro_use]
extern crate dotenv_codegen;

use openai::*;
use serde::{ Serialize, Deserialize };
use std::fs::{ write, read_to_string };

pub mod openai;

#[derive(Serialize, Deserialize)]
struct TrainingData {
    pub prompt: String,
    pub completion: String,
}

pub async fn prompt(message: &str, user: &str, history: Vec<(String, String)>) -> String {
    let mut history_string = String::new();

    for message in history {
        history_string.insert_str(0, &format!("{}: {}\n", message.0, message.1));
    }

    let response = create_completion(CreateCompletionRequestBody {
        model: String::from("davinci:ft-personal-2022-12-12-04-49-51"),
        prompt: Some(format!("{history_string}{user}: {message}\nAI-bert: ")),
        max_tokens: Some(1024),
        stop: Some(String::from("\n")),
        user: Some(String::from(user)),
        ..CreateCompletionRequestBody::default()
    }).await;

    dbg!(&response);

    response.choices.first().expect("No choices returned").text.to_string()
}

async fn deserialize_training_data() -> Vec<TrainingData> {
    let training_file_contents = read_to_string(dotenv!("TRAINING_FILE"))
        .expect("Failed to read training file");

    serde_json::from_str(&training_file_contents)
        .expect("Failed to deserialize training file contents")
}

pub async fn begin_training_history(event: &str) {
    let mut training_data = deserialize_training_data().await;
    let event_training_data = TrainingData {
        prompt: format!("History:\n{}\n\n###\n\n", event),
        completion: String::from(" \n")
    };

    training_data.push(event_training_data);

    let serialized_training_data = serde_json::to_string_pretty(&training_data)
        .expect("Failed to serialize training data");

    write(dotenv!("TRAINING_FILE"), serialized_training_data)
        .expect("Error writing training data");
}

pub async fn respond_to_training_history(response: &str) {
    let mut training_data = deserialize_training_data().await;
    let last_event = training_data.last_mut()
        .expect("There is no training data");

    last_event.completion = format!(" {}\n", response);

    write(
        dotenv!("TRAINING_FILE"),
        serde_json::to_string_pretty(&training_data).expect("Failed to re-serialize training data")
    ).expect("Failed to write to training file");
}

pub async fn add_training_event(event: &str) {
    let mut training_data = deserialize_training_data().await;
    let last_entry = training_data.last().expect("No training data");
    let mut last_prompt = last_entry.prompt.clone();
    let last_completion = &last_entry.completion;
    let last_response = &last_completion[1..last_completion.len() - 1];

    if !last_response.is_empty() {
        last_prompt.insert_str(
            last_prompt.len() - 7,
            &format!("\nAI-bert responded: {}\n{event}", last_response),
        );
    } else {
        last_prompt.insert_str(
            last_prompt.len() - 7,
            &format!("\n{event}"),
        );
    }

    training_data.push(TrainingData {
        prompt: last_prompt,
        completion: String::from(" \n"),
    });

    write(
        dotenv!("TRAINING_FILE"),
        serde_json::to_string_pretty(&training_data)
            .expect("Failed to re-serialize training data"),
    )
        .expect("Failed to write to training file");
}
