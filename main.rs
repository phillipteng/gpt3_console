use std::time::Instant;
use std::io::{self, BufRead};
use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GPT3Request {
    model: String,
    prompt: String,
    max_tokens: usize,
    temperature: f32,
}

#[derive(Deserialize)]
struct GPT3Response {
    choices: Vec<GPT3Choice>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct GPT3Choice {
    text: String,
    index: usize,
    logprobs: Option<GPT3LogProbs>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct GPT3LogProbs {
    tokens: Vec<String>,
    token_logprobs: Vec<f32>,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Set up the HTTP client and headers
    let http_client = reqwest::blocking::Client::new();
    let mut headers = HeaderMap::new();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap();
    headers.insert(AUTHORIZATION, auth_header_value);
    let content_header_value = HeaderValue::from_str(&format!("application/json")).unwrap();
    headers.insert(CONTENT_TYPE, content_header_value);
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        println!("What do you want to ask?");
        stdin.lock().read_line(&mut input).unwrap();
        // Create a request to the GPT-3 API
        // Change the model that you want to use here
        let request = GPT3Request {
            model: "text-davinci-003".to_string(),
            prompt: input.to_string(),
            max_tokens: 4000,
            temperature: 0.0,
        };
        input.clear();
        // let response: GPT3Response = http_client.post("https://api.openai.com/v1/engines/davinci-codex/completions")
        let start_time = Instant::now();
        let response: GPT3Response = http_client.post("https://api.openai.com/v1/completions")
            .headers(headers.clone())
            .json(&request)
            .send()
            .unwrap()
            .json()
            .unwrap();

        let end_time = Instant::now();
        let duration = end_time - start_time;
        let text = response.choices[0].text.trim();
        println!("{}", text);
        println!("The request took {} seconds.", duration.as_secs_f32());
    }
}

