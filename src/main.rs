use clap::{App, Arg};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("CLI App")
        .version("1.0")
        .author("Your Name")
        .about("Calls an API with your text")
        .arg(Arg::with_name("text")
             .help("The text to send to the API")
             .required(true)
             .index(1))
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let client = Client::new();
    let response = client.post("http://localhost:5000/v1/chat/completions")
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "phi-2.Q4_0.gguf",
            "messages": [
                {"role": "system", "content": "You are a cli assistant, you will write commands for powershell and answer questions about syntax if asked. If asked for a command you will respond with only the command."},
                {"role": "user", "content": text}
            ]
        }))
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    if let Some(choice) = response.choices.get(0) {
        println!("{}", choice.message.content);
    }

    Ok(())
}
