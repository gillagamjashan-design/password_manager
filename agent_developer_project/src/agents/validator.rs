use crate::ai_client::call_claude;
use crate::messages::{FinalPayload, ValidationPayload};

/// The Validator agent checks whether the code actually addresses the user's task.
/// Powered by: Claude (Anthropic) — Testing specialist.
pub struct ValidatorAgent;

impl ValidatorAgent {
    pub fn new() -> Self { ValidatorAgent }

    pub fn process(&self, result: &FinalPayload, task_description: &str) -> ValidationPayload {
        println!("\n\x1b[1;33m[VALIDATOR]\x1b[0m Checking output matches task...");
        println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[2m· Brain: Claude (Testing)\x1b[0m");

        let (passed, reason) = self.validate(&result.code, task_description);

        if passed {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mValidation passed.\x1b[0m");
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[31mValidation failed: {reason}\x1b[0m");
        }

        ValidationPayload { task_id: result.task_id, passed, reason }
    }

    fn validate(&self, code: &str, task_description: &str) -> (bool, String) {
        let system = "You are a software testing specialist. \
            Determine if the provided Rust code correctly addresses the given task. \
            Reply with exactly one of:\n\
            PASS: <one sentence explaining why it passes>\n\
            FAIL: <one sentence explaining what is wrong>";

        let user = format!(
            "Task: {task_description}\n\nCode:\n{code}\n\nDoes this code correctly implement the task?"
        );

        match call_claude(system, &user) {
            Ok(response) => self.parse_validation(&response),
            Err(e) => {
                println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[33mClaude unavailable: {e}\x1b[0m");
                (self.keyword_check(task_description, code), "Fallback keyword check used.".to_string())
            }
        }
    }

    fn parse_validation(&self, response: &str) -> (bool, String) {
        let trimmed = response.trim();
        if trimmed.to_uppercase().starts_with("PASS") {
            let reason = trimmed
                .trim_start_matches(|c: char| c.is_uppercase())
                .trim_start_matches(':')
                .trim();
            (true, reason.to_string())
        } else {
            let reason = trimmed
                .trim_start_matches(|c: char| c.is_uppercase())
                .trim_start_matches(':')
                .trim();
            (false, reason.to_string())
        }
    }

    fn keyword_check(&self, task: &str, code: &str) -> bool {
        let stop_words = ["a", "an", "the", "write", "create", "make", "function", "in", "for"];
        let task_words: Vec<String> = task
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !stop_words.contains(w) && w.len() >= 3)
            .map(|w| w.to_string())
            .collect();
        let code_lower = code.to_lowercase();
        task_words.is_empty() || task_words.iter().any(|w| code_lower.contains(w.as_str()))
    }
}
