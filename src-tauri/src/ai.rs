use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemContext {
    pub app: String,
    pub clipboard: String,
}

#[tauri::command]
pub async fn get_ai_response(context: SystemContext) -> Result<String, String> {
    println!("🧠 Context: {:?}", context);

    let prompt = format!(
        "Active App: {}\nClipboard: {}\n\nSuggest a helpful action.",
        context.app, context.clipboard
    );

    println!("✍️ Prompt:\n{}", prompt);

    let json_body = serde_json::json!({
        "model": "mistral",
        "prompt": prompt,
        "stream": false
    });

    let res = Client::new()
        .post("http://localhost:11434/api/generate")
        .json(&json_body)
        .send()
        .await
        .map_err(|e| {
            println!("❌ HTTP Error: {}", e);
            format!("HTTP error: {}", e)
        })?;

    let json = res.json::<serde_json::Value>().await.map_err(|e| {
        println!("❌ JSON Parse Error: {}", e);
        e.to_string()
    })?;

    println!("✅ Ollama Response: {:?}", json);

    let response = json["response"]
        .as_str()
        .unwrap_or("No response.")
        .to_string();

    Ok(response)
}
