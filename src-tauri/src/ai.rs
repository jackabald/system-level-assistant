use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemContext {
    pub app: String,
    pub clipboard: String,
}

fn build_prompt(app: &str, clip: &str) -> String {
    if app.contains("Gmail") {
        format!("You are writing an email.\nClipboard:\n\"{}\"\n\nSuggest improvements in 3 bullets.", clip)
    } else {
        format!(
            "User is using '{}'. Clipboard text:\n\"{}\"\n\nReply with one short helpful suggestion.",
            app, clip
        )
    }
}

#[tauri::command]
pub async fn get_ai_response(context: SystemContext) -> Result<String, String> {
    let prompt = build_prompt(&context.app, &context.clipboard);
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|e| e.to_string())?;
    let json_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "system", "content": "You are a concise desktop assistant."},
            {"role": "user", "content": prompt}
        ],
        "max_tokens": 120,
        "temperature": 0.6
    });

    let reply = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json_body)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?
        ["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response.")
        .to_string();

    Ok(reply)
}
