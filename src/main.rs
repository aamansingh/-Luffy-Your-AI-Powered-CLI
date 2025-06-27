use std::env;
use std::io::{self, Write};

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("HF_API_KEY").expect("HF_API_KEY not set in .env");

    let client = Client::new();

    println!("🤖 Hugging Face Chatbot");
    println!("Type 'exit' to quit.");

    let stdin = io::stdin();

    loop {
        print!("\n💬 You: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        let request_body = HFRequest {
            inputs: user_input.to_string(),
        };

        let response = client
            .post("https://api-inference.huggingface.co/models/mistralai/Mistral-7B-Instruct-v0.2")
            .bearer_auth(&api_key)
            .json(&request_body)
            .send()
            .await
            .expect("Request failed");

        let status = response.status();
        let text = response.text().await.expect("Failed to read response");

        if !status.is_success() {
            println!("⚠️ Request failed: HTTP {} - {}", status, text);
            continue;
        }

        // Print raw response for debugging
        println!("\n🔍 Raw response:\n{}\n", text);

        // Some models return plain text, some return array of dicts
        if text.trim_start().starts_with("{") {
            // Try to parse as JSON object (e.g., error)
            let response_json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| {
                serde_json::json!({"error":"Could not parse JSON"})
            });

            if let Some(err) = response_json.get("error") {
                println!("⚠️ API error: {}", err);
                continue;
            }

            println!("⚠️ Unknown JSON response: {}", response_json);
            continue;
        }

        if text.trim_start().starts_with("[") {
            // Try to parse as JSON array (typical response)
            let response_json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| {
                serde_json::json!([{"generated_text":"Could not parse response"}])
            });

            let bot_reply = response_json[0]["generated_text"]
                .as_str()
                .unwrap_or("Sorry, no response.")
                .trim();

            println!("🤖 Bot: {}", bot_reply);
        } else {
            // Fallback plain text
            println!("🤖 Bot: {}", text.trim());
        }
    }
}

