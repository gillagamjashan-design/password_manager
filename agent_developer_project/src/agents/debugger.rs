use crate::ai_client::call_deepseek;
use crate::messages::{FinalPayload, ReviewPayload};

/// The Debugger agent fixes issues found by the Reviewer and optimizes the code.
/// Powered by: DeepSeek-Coder — Debugging & Optimization specialist.
pub struct DebuggerAgent;

impl DebuggerAgent {
    pub fn new() -> Self { DebuggerAgent }

    pub fn process(&self, review: ReviewPayload) -> FinalPayload {
        println!("\n\x1b[1;31m[DEBUGGER]\x1b[0m \x1b[2m· Brain: DeepSeek-Coder (Debugging & Optimization)\x1b[0m");

        if review.approved {
            println!("\x1b[1;31m[DEBUGGER]\x1b[0m Code already approved. Checking for optimizations...");
        } else {
            println!("\x1b[1;31m[DEBUGGER]\x1b[0m Fixing {} issue(s)...", review.issues.len());
        }

        let (fixed_code, summary) = self.fix_and_optimize(&review);

        println!("\x1b[1;31m[DEBUGGER]\x1b[0m {}", summary);
        println!("\x1b[1;31m[DEBUGGER]\x1b[0m Handing final code to Coordinator.");

        FinalPayload {
            task_id: review.task_id,
            code: fixed_code,
            summary,
        }
    }

    fn fix_and_optimize(&self, review: &ReviewPayload) -> (String, String) {
        let issues_text = if review.issues.is_empty() {
            "No issues found — optimize for performance and clarity if possible.".to_string()
        } else {
            format!(
                "Fix these issues:\n{}",
                review.issues.iter().map(|i| format!("- {i}")).collect::<Vec<_>>().join("\n")
            )
        };

        let system = "You are a Rust debugging and optimization specialist powered by DeepSeek-Coder. \
            Fix all issues in the provided code and optimize it for performance and clarity. \
            Return ONLY the fixed Rust code — no markdown, no backticks, no explanation.";

        let user = format!(
            "Code to fix/optimize:\n\n{}\n\n{}",
            review.code, issues_text
        );

        match call_deepseek(system, &user) {
            Ok(fixed) => {
                let clean = self.clean_code_response(&fixed);
                let summary = if review.approved {
                    "Code optimized by DeepSeek-Coder.".to_string()
                } else {
                    format!("Fixed {} issue(s) and optimized with DeepSeek-Coder.", review.issues.len())
                };
                (clean, summary)
            }
            Err(e) => {
                println!("\x1b[1;31m[DEBUGGER]\x1b[0m \x1b[33mDeepSeek unavailable: {e}\x1b[0m");
                (review.code.clone(), "DeepSeek unavailable — original code passed through.".to_string())
            }
        }
    }

    fn clean_code_response(&self, code: &str) -> String {
        let trimmed = code.trim();
        if trimmed.starts_with("```") {
            let inner = trimmed
                .trim_start_matches("```rust")
                .trim_start_matches("```")
                .trim_end_matches("```");
            return inner.trim().to_string();
        }
        trimmed.to_string()
    }
}
