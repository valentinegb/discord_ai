use reqwest::{ Client, header::AUTHORIZATION };
use dotenv::dotenv;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct TextCompletion {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<TextCompletionChoice>,
    pub usage: TextCompletionUsage,
}

#[derive(Deserialize, Debug)]
pub struct TextCompletionChoice {
    pub text: String,
    pub index: i32,
    pub logprobs: Option<i32>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct TextCompletionUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: Option<i32>,
    pub total_tokens: i32,
}

#[derive(Serialize, Default)]
pub struct CreateCompletionRequestBody {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

pub async fn create_completion(body: CreateCompletionRequestBody) -> TextCompletion {
    dotenv().expect("Error initializing dotenv");

    let client = Client::new();

    client
        .post("https://api.openai.com/v1/completions")
        .header(AUTHORIZATION, format!("Bearer {}", dotenv!("OPENAI_API_KEY")))
        .json(&body)
        .send()
        .await
        .expect("Error sending request")
        .json()
        .await
        .expect("Error deserializing response")
}
