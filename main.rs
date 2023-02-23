use std::io::{stdout, Write};
use std::thread::{spawn, sleep};
use std::sync::{Arc, Mutex};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, BufRead};
use std::time::Duration;
use std::time::Instant;

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
    // Set up the HTTP client and headers and timeout duration to 60 seonds
    let http_client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap();
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
        let request = GPT3Request {
            model: "text-davinci-003".to_string(),
            prompt: input.to_string(),
            max_tokens: 2000,
            temperature: 0.0,
        };
        input.clear();
        
        let should_stop = Arc::new(Mutex::new(false));

        // Spawn a new thread to print the animation
        let handle = spawn({
            let should_stop = should_stop.clone();
            move || {
                // Define the animation sequence as a string
                let animation = "|/-\\";

                // Loop through the animation sequence until the stop flag is set
                loop {
                    // Check the stop flag and exit the loop if it's set
                    if *should_stop.lock().unwrap() {
                        break;
                    }

                    // Print the next character in the sequence
                    for c in animation.chars() {
                        stdout().flush().unwrap();
                        print!("{}", c);
                        stdout().flush().unwrap();
                        sleep(Duration::from_millis(100));
                        print!("\r");
                    }
                }
            }
        });

        let start_time = Instant::now();
        let response: GPT3Response = http_client
            .post("https://api.openai.com/v1/completions")
            .headers(headers.clone())
            .json(&request)
            .send()
            .unwrap()
            .json()
            .unwrap();

        *should_stop.lock().unwrap() = true;
        handle.join().unwrap();
        let end_time = Instant::now();
        let duration = end_time - start_time;
        let text = response.choices[0].text.trim();
        println!("{}", text);
        println!("The request took {} seconds.", duration.as_secs_f32());
    }
}

