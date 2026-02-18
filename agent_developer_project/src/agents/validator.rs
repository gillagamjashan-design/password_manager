use crate::messages::{FinalPayload, ValidationPayload};

/// The Validator agent checks whether the final code actually addresses the user's task.
/// This prevents the "says done but ignored the task" bug.
/// It uses heuristic keyword matching between the task description and the code output.
pub struct ValidatorAgent;

impl ValidatorAgent {
    pub fn new() -> Self {
        ValidatorAgent
    }

    /// Checks whether the code output is relevant to the task description.
    /// Returns a ValidationPayload with pass/fail and a reason.
    pub fn process(&self, result: &FinalPayload, task_description: &str) -> ValidationPayload {
        println!("\n\x1b[1;33m[VALIDATOR]\x1b[0m Checking output matches task...");

        let relevant = self.is_output_relevant(task_description, &result.code);

        if relevant {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mOutput matches task. Validation passed.\x1b[0m");
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[31mOutput does NOT match task. Sending back for retry.\x1b[0m");
        }

        ValidationPayload {
            task_id: result.task_id,
            passed: relevant,
            reason: if relevant {
                "Code contains task-relevant keywords and structure.".to_string()
            } else {
                format!(
                    "Code does not appear to address: \"{}\". Retrying with more context.",
                    task_description
                )
            },
        }
    }

    /// Heuristic: extract meaningful words from the task description and check
    /// whether any of them appear in the generated code (function names, comments, etc.)
    fn is_output_relevant(&self, task_description: &str, code: &str) -> bool {
        // Skip common filler words
        let stop_words = [
            "a", "an", "the", "that", "this", "is", "are", "was", "were", "be",
            "been", "being", "have", "has", "had", "do", "does", "did", "will",
            "would", "shall", "should", "may", "might", "must", "can", "could",
            "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "from", "up", "about", "into", "through", "write", "function",
            "create", "make", "build", "implement", "code", "program",
        ];

        let task_words: Vec<String> = task_description
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !stop_words.contains(w))
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| w.len() >= 3)
            .collect();

        let code_lower = code.to_lowercase();

        // If we got no meaningful words, pass (nothing to check against)
        if task_words.is_empty() {
            return true;
        }

        // Pass if at least one meaningful task word appears in the code
        task_words.iter().any(|word| code_lower.contains(word.as_str()))
    }
}
