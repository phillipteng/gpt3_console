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


## User Manual

### Setup your API Key
Get your API Key here:
https://platform.openai.com/account/api-keys

Then add the key to your .bashrc or .zshrc file:
```
export OPENAI_API_KEY='YOUR_KEY_HERE'
```
* for more information follow instructions here: https://help.openai.com/en/articles/5112595-best-practices-for-api-key-safety

To run the program:
$ cargo run 
To build for release:
$ cargo build --release

The program will prompt what you want to ask or request the GPT-3 Model. After a couple seconds, the output along with the time elapsed will print into the console.

### Additional Settings examples
You can set additional models available that might be faster than davinci-003 here:
https://platform.openai.com/docs/models/overview

You can also change `max_tokens` that changes the maximum length of the response as denoted in the link above.
