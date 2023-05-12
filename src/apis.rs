use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
    CreateChatCompletionRequestArgs, CreateChatCompletionResponse, Role,
};
use async_openai::Client as OpenAiClient;

use std::fs;

pub async fn gpt_query(
    model: &str,
    max_tokens: u16,
    query: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let openai_api_key = read_api_key_from_credentials_file()?;
    let openai = OpenAiClient::new().with_api_key(openai_api_key);
    let messages = build_messages_from_query(query);
    let response = query_gpt_with_retries(&openai, model, &messages, max_tokens, 3).await?;
    let result = response.choices.first().unwrap().message.content.clone();
    Ok(result)
}

fn read_api_key_from_credentials_file() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("credentials.toml")?;
    let cred: toml::Value = contents.parse()?;
    let openai_api_key = cred["openai_api_key"]
        .as_str()
        .ok_or("Expected a openai_api_key in the credentials.toml file")?
        .to_string();
    Ok(openai_api_key)
}

fn build_messages_from_query(
    query: &str,
) -> std::vec::Vec<async_openai::types::ChatCompletionRequestMessage> {
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
    messages
}

async fn query_gpt_with_retries(
    openai: &OpenAiClient,
    model: &str,
    messages: &[ChatCompletionRequestMessage],
    max_tokens: u16,
    retries: usize,
) -> Result<CreateChatCompletionResponse, Box<dyn std::error::Error>> {
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages.to_vec())
        .max_tokens(max_tokens)
        .build()
        .unwrap();

    let mut tries = 0;
    while tries < retries {
        let result = openai.chat().create(request.clone()).await;
        if result.is_ok() {
            return Ok(result.unwrap());
        }
        tries += 1;
    }
    Err("Failed to get response from openai")?
}
