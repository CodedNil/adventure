use std::fs::File;
use std::io::prelude::*;

use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
    CreateChatCompletionRequestArgs, Role,
};
use async_openai::Client as OpenAiClient;

fn get_credentials() -> toml::Value {
    // Read credentials.toml file to get keys
    let mut file = File::open("credentials.toml").expect("Failed to open credentials file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read credentials file");
    let cred: toml::Value = contents.parse().expect("Failed to parse credentials TOML");

    cred
}

/// Get openai client
fn get_openai() -> OpenAiClient {
    let cred = get_credentials();

    // Configure the client with your openai api key
    let openai_api_key = cred["openai_api_key"]
        .as_str()
        .expect("Expected a openai_api_key in the credentials.toml file")
        .to_string();
    OpenAiClient::new().with_api_key(openai_api_key)
}

/// Use gpt to query information
pub async fn gpt_info_query(model: &str, max_tokens: u16, query: &str) -> Result<String, String> {
    let openai = get_openai();

    // Turn query into a array of ChatCompletionRequestMessageArgs, each line starts with S: A: or U: for system assistant or user
    let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();
    for line in query.split("###").filter(|s| !s.trim().is_empty()) {
        let (role, content) = match line.chars().next().unwrap() {
            'S' => (Role::System, &line[2..]),
            'A' => (Role::Assistant, &line[2..]),
            'U' => (Role::User, &line[2..]),
            _ => (Role::User, line),
        };
        let message = ChatCompletionRequestMessageArgs::default()
            .role(role)
            .content(content.trim().to_string())
            .build()
            .unwrap();
        messages.push(message);
    }

    // Search with gpt through the memories to answer the query
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .max_tokens(max_tokens)
        .build()
        .unwrap();

    // Retry the request if it fails
    let mut tries = 0;
    let response = loop {
        let response = openai.chat().create(request.clone()).await;
        if let Ok(response) = response {
            break Ok(response);
        }
        tries += 1;
        if tries >= 3 {
            break response;
        }
    };
    // Return from errors
    if response.is_err() {
        return Err("Failed to get response from openai".to_string());
    }
    let result = response
        .unwrap()
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone();
    Ok(result)
}
