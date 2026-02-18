use crate::ai_client::call_claude;
use crate::messages::{CodePayload, ReviewPayload};

/// The Reviewer agent checks code for security issues and documentation quality.
/// Powered by: Claude (Anthropic) — Security & Docs specialist.
pub struct ReviewerAgent;

impl ReviewerAgent {
    pub fn new() -> Self { ReviewerAgent }

    pub fn process(&self, code_payload: CodePayload) -> ReviewPayload {
        println!("\n\x1b[1;35m[REVIEWER]\x1b[0m Received code for review. Analyzing...");
        println!("\x1b[1;35m[REVIEWER]\x1b[0m \x1b[2m· Brain: Claude (Security & Docs)\x1b[0m");

        let (issues, approved) = self.review_code(&code_payload.code);

        if approved {
            println!("\x1b[1;35m[REVIEWER]\x1b[0m No issues found. Code approved.");
        } else {
            println!("\x1b[1;35m[REVIEWER]\x1b[0m Found {} issue(s):", issues.len());
            for issue in &issues {
                println!("\x1b[1;35m[REVIEWER]\x1b[0m   - {}", issue);
            }
        }
        println!("\x1b[1;35m[REVIEWER]\x1b[0m Handing off to Debugger.");

        ReviewPayload {
            task_id: code_payload.task_id,
            code: code_payload.code,
            issues,
            approved,
        }
    }

    fn review_code(&self, code: &str) -> (Vec<String>, bool) {
        let system = "You are a Rust code security and documentation reviewer. \
            Review the code for: security issues, missing comments, missing main(), \
            excessive unwrap() usage, and correctness. \
            If the code is good, reply with exactly: APPROVED\
            If there are issues, list them one per line starting with '- '. \
            Be concise. No extra explanation.";

        let user = format!("Review this Rust code:\n\n{code}");

        match call_claude(system, &user) {
            Ok(response) => self.parse_review(&response),
            Err(e) => {
                println!("\x1b[1;35m[REVIEWER]\x1b[0m \x1b[33mClaude unavailable: {e}\x1b[0m");
                (self.fallback_check(code), false)
            }
        }
    }

    fn parse_review(&self, response: &str) -> (Vec<String>, bool) {
        let trimmed = response.trim();
        if trimmed.to_uppercase().contains("APPROVED") && !trimmed.contains("- ") {
            return (vec![], true);
        }
        let issues: Vec<String> = trimmed
            .lines()
            .filter(|l| l.trim().starts_with("- ") || l.trim().starts_with("* "))
            .map(|l| l.trim().trim_start_matches("- ").trim_start_matches("* ").to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if issues.is_empty() {
            (vec![], true)
        } else {
            (issues, false)
        }
    }

    fn fallback_check(&self, code: &str) -> Vec<String> {
        let mut issues = vec![];
        if !code.contains("//") && !code.contains("///") {
            issues.push("Missing comments".to_string());
        }
        if !code.contains("fn main()") {
            issues.push("Missing main() function".to_string());
        }
        issues
    }
}
