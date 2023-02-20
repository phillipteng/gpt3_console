# GPT3_Console_Rust
A simple console application that performs API calls to OpenAI models (including GPT-3) and displays the output. The program is written with rust lang.

## Installation
Prerequisites:
Install rust and cargo.

Run the following shell commands:
$ cargo init

Then inside the created Cargo.toml file, append the following after `[dependencies]`:
```
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
```

To build:
$ cargo run 
To build for release:
$ cargo build --release

## User Manual
The program will prompt what you want to ask or request the GPT-3 Model. After a couple seconds, the output along with the time elapsed will print into the console.

### Additional Settings examples
You can set additional models available that might be faster than davinci-003 here:
https://platform.openai.com/docs/models/overview

You can also change `max_tokens` that changes the maximum length of the response as denoted in the link above.
