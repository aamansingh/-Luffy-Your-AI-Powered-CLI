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



















// use std::env;
// use dotenv::dotenv;
// use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
// use serde::{Deserialize, Serialize};
// use reqwest::multipart;
// use std::fs::File;
// use std::io::Read;
// // use tokio_util::codec::{BytesCodec, FramedRead};
// // use tokio_util::io::ReaderStream;
// use tts::Tts;
// use std::io::{self, Write};

// fn speak(text: &str) {
//     let mut tts = Tts::default().unwrap();
//     tts.speak(text, false).unwrap();
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();

//     println!("🧠 Luffy CLI - Your AI Assistant");
//     println!("💬 Ask me anything:");

//     let mut conversation_history: Vec<ChatMessage> = Vec::new();

//     loop {
//         print!("\n💬 You: ");
//         io::stdout().flush().unwrap();

//         let audio_path = "input.wav"; // Replace with your real audio file path
//         let api_key = env::var("OPENAI_API_KEY")?; // Use correct env var name!
//         let user_input = transcribe_audio("input.wav", &hf_api_key).await?;
//     let ai_response = ask_huggingface(&user_input, &hf_api_key).await?;

//         let input = user_input;

//         if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
//             println!("👋 Goodbye from Luffy!");
//             break;
//         }

//         if input.is_empty() {
//             continue;
//         }

//         // Add user message to history
//         conversation_history.push(ChatMessage {
//             role: "user".to_string(),
//             content: input.to_string(),
//         });

//         match ask_openai(&conversation_history).await {
//             Ok(response) => {
//                 println!("🤖 Luffy: {}", response);
//                 speak(&response);
//                 conversation_history.push(ChatMessage {
//                     role: "assistant".to_string(),
//                     content: response,
//                 });
//             }
//             Err(e) => eprintln!("❌ Error: {}", e),
//         }
//     }

//     Ok(())
// }


// pub async fn ask_huggingface(prompt: &str, hf_api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let body = serde_json::json!({
//         "inputs": prompt
//     });

//     let client = reqwest::Client::new();
//     let res = client
//         .post("https://api-inference.huggingface.co/models/mistralai/Mistral-7B-Instruct-v0.1")
//         .header("Authorization", format!("Bearer {}", hf_api_key))
//         .json(&body)
//         .send()
//         .await?;

//     let json: serde_json::Value = res.json().await?;
//     let answer = json[0]["generated_text"].as_str().unwrap_or("").to_string();
//     Ok(answer)
// }


// // async fn ask_openai(messages: &Vec<ChatMessage>) -> Result<String, Box<dyn std::error::Error>> {
// //     let api_key = env::var("OPENAI_API_KEY")?;
// //     let client = reqwest::Client::new();

// //     let request_body = OpenAIRequest {
// //         model: "gpt-4o".to_string(), // or "gpt-4o-mini"
// //         messages: messages.clone(),
// //     };

// //     let res = client
// //         .post("https://api.openai.com/v1/chat/completions")
// //         .header(AUTHORIZATION, format!("Bearer {}", api_key))
// //         .header(CONTENT_TYPE, "application/json")
// //         .json(&request_body)
// //         .send()
// //         .await?;

// //     let res_json: OpenAIResponse = res.json().await?;
// //     Ok(res_json.choices[0].message.content.clone())
// // }

// pub async fn transcribe_audio(file_path: &str, hf_api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let audio_data = std::fs::read(file_path)?;

//     let client = reqwest::Client::new();
//     let res = client
//         .post("https://api-inference.huggingface.co/models/openai/whisper-large")
//         .header("Authorization", format!("Bearer {}", hf_api_key))
//         .header("Content-Type", "audio/wav")
//         .body(audio_data)
//         .send()
//         .await?;

//     let json: serde_json::Value = res.json().await?;
//     let transcription = json["text"].as_str().unwrap_or("").to_string();
//     Ok(transcription)
// }


// #[derive(Serialize)]
// struct OpenAIRequest {
//     model: String,
//     messages: Vec<ChatMessage>,
// }

// #[derive(Serialize, Clone)]
// struct ChatMessage {
//     role: String,
//     content: String,
// }

// #[derive(Deserialize)]
// struct OpenAIResponse {
//     choices: Vec<OpenAIChoice>,
// }

// #[derive(Deserialize)]
// struct OpenAIChoice {
//     message: ChatMessageResponse,
// }

// #[derive(Deserialize)]
// struct ChatMessageResponse {
//     content: String,
// }
