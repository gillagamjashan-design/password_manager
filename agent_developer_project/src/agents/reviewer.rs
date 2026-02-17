use crate::messages::{CodePayload, ReviewPayload};

/// The Reviewer agent checks code for quality issues.
/// Think of this as a senior engineer doing a code review before merging.
pub struct ReviewerAgent;

impl ReviewerAgent {
    pub fn new() -> Self {
        ReviewerAgent
    }

    /// Reviews the code and returns feedback.
    /// If no issues are found, the code is approved.
    pub fn process(&self, code_payload: CodePayload) -> ReviewPayload {
        println!("\n[REVIEWER] Received code for review. Analyzing...");

        let issues = self.check_for_issues(&code_payload.code);

        if issues.is_empty() {
            println!("[REVIEWER] No issues found. Code looks good!");
            println!("[REVIEWER] Approved. Handing off to Coordinator.");
        } else {
            println!("[REVIEWER] Found {} issue(s):", issues.len());
            for issue in &issues {
                println!("[REVIEWER]   - {}", issue);
            }
            println!("[REVIEWER] Sending to Debugger for fixes.");
        }

        ReviewPayload {
            task_id: code_payload.task_id,
            code: code_payload.code,
            issues: issues.clone(),
            approved: issues.is_empty(),
        }
    }

    /// Checks the code for common quality issues.
    /// Returns a list of issue descriptions (empty = no issues).
    fn check_for_issues(&self, code: &str) -> Vec<String> {
        let mut issues = Vec::new();

        // Check: does the code have at least one comment?
        if !code.contains("///") && !code.contains("//") {
            issues.push("Missing comments: code should be documented".to_string());
        }

        // Check: does it have a main() function?
        if !code.contains("fn main()") {
            issues.push("Missing main() function: code cannot be run as-is".to_string());
        }

        // Check: are there any println! calls for output?
        if !code.contains("println!") {
            issues.push("No output: code should print results so users can verify it works".to_string());
        }

        // Check: does it use unwrap() without explanation? (can panic)
        let unwrap_count = code.matches(".unwrap()").count();
        if unwrap_count > 2 {
            issues.push(format!(
                "Excessive unwrap() usage ({}): consider adding error handling comments",
                unwrap_count
            ));
        }

        issues
    }
}
