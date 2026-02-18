//! AI client — HTTP functions for calling OpenAI, DeepSeek, and Anthropic APIs.
//!
//! HOW TO USE:
//! Set these environment variables in your terminal before running:
//!   export OPENAI_API_KEY="your-key"
//!   export DEEPSEEK_API_KEY="your-key"
//!   export ANTHROPIC_API_KEY="your-key"
//!
//! Each function takes a system prompt (the agent's role/instructions) and a
//! user prompt (the actual task content), and returns the AI's response text.

use serde_json::{json, Value};

/// Result type used across all AI calls.
pub type AiResult = Result<String, String>;

// ─── OpenAI (GPT-4o) ─────────────────────────────────────────────────────────

/// Calls GPT-4o (OpenAI). Used by: CoderAgent.
/// Role: Coding specialist — writes high-quality Rust code.
pub fn call_gpt(system: &str, user: &str) -> AiResult {
    let key = std::env::var("OPENAI_API_KEY").map_err(|_| {
        "Missing OPENAI_API_KEY. Get one at platform.openai.com, then run:\n  export OPENAI_API_KEY=\"your-key\"".to_string()
    })?;

    let body = json!({
        "model": "gpt-4o",
        "messages": [
            { "role": "system", "content": system },
            { "role": "user",   "content": user   }
        ],
        "max_tokens": 2000,
        "temperature": 0.2
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(key)
        .json(&body)
        .send()
        .map_err(|e| format!("OpenAI request failed: {e}"))?;

    let json: Value = resp.json().map_err(|e| format!("OpenAI parse failed: {e}"))?;
    extract_openai_text(&json)
}

// ─── DeepSeek (deepseek-coder) ───────────────────────────────────────────────

/// Calls DeepSeek-Coder. Used by: DebuggerAgent.
/// Role: Debugging and optimization specialist.
pub fn call_deepseek(system: &str, user: &str) -> AiResult {
    let key = std::env::var("DEEPSEEK_API_KEY").map_err(|_| {
        "Missing DEEPSEEK_API_KEY. Get one at platform.deepseek.com, then run:\n  export DEEPSEEK_API_KEY=\"your-key\"".to_string()
    })?;

    // DeepSeek uses the same request format as OpenAI
    let body = json!({
        "model": "deepseek-coder",
        "messages": [
            { "role": "system", "content": system },
            { "role": "user",   "content": user   }
        ],
        "max_tokens": 2000,
        "temperature": 0.1
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.deepseek.com/chat/completions")
        .bearer_auth(key)
        .json(&body)
        .send()
        .map_err(|e| format!("DeepSeek request failed: {e}"))?;

    let json: Value = resp.json().map_err(|e| format!("DeepSeek parse failed: {e}"))?;
    extract_openai_text(&json)
}

// ─── Anthropic (Claude) ──────────────────────────────────────────────────────

/// Calls Claude (Anthropic). Used by: PlannerAgent, ReviewerAgent, ValidatorAgent, CoordinatorAgent.
/// Roles: Architecture, Security & Docs, Testing.
pub fn call_claude(system: &str, user: &str) -> AiResult {
    let key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| {
        "Missing ANTHROPIC_API_KEY. Get one at console.anthropic.com, then run:\n  export ANTHROPIC_API_KEY=\"your-key\"".to_string()
    })?;

    let body = json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 2000,
        "system": system,
        "messages": [
            { "role": "user", "content": user }
        ]
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("Anthropic request failed: {e}"))?;

    let json: Value = resp.json().map_err(|e| format!("Anthropic parse failed: {e}"))?;
    extract_claude_text(&json)
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extracts the assistant's reply text from OpenAI / DeepSeek JSON response.
fn extract_openai_text(json: &Value) -> AiResult {
    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Unexpected API response format: {json}"))
}

/// Extracts the assistant's reply text from Anthropic JSON response.
fn extract_claude_text(json: &Value) -> AiResult {
    json["content"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Unexpected Anthropic response format: {json}"))
}
