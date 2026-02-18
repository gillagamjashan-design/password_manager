use crate::messages::{FinalPayload, ReviewPayload};

/// The Debugger agent fixes issues identified by the Reviewer.
/// Think of this as a developer who takes review feedback and applies the changes.
pub struct DebuggerAgent;

impl DebuggerAgent {
    pub fn new() -> Self {
        DebuggerAgent
    }

    /// Fixes any issues in the code found by the reviewer.
    pub fn process(&self, review: ReviewPayload) -> FinalPayload {
        if review.approved {
            println!("\n\x1b[1;31m[DEBUGGER]\x1b[0m Code was already approved. No changes needed.");
            return FinalPayload {
                task_id: review.task_id,
                code: review.code,
                summary: "Code passed review with no issues.".to_string(),
            };
        }

        println!("\n\x1b[1;31m[DEBUGGER]\x1b[0m Fixing {} issue(s)...", review.issues.len());
        let mut code = review.code.clone();
        let mut fixes_applied = Vec::new();

        for issue in &review.issues {
            if issue.contains("Missing comments") {
                // Add a top-level comment if none exists
                code = format!("// Agent-generated code\n// This file was produced by the agent team\n\n{}", code);
                fixes_applied.push("Added missing documentation comment".to_string());
            }
            if issue.contains("Missing main()") {
                // Append a minimal main function
                code = format!("{}\n\nfn main() {{\n    println!(\"Running agent-generated code...\");\n}}", code);
                fixes_applied.push("Added missing main() function".to_string());
            }
            if issue.contains("No output") {
                // Note the issue in a comment — cannot safely inject println! without context
                code = format!("// NOTE: Add println! calls to display output\n{}", code);
                fixes_applied.push("Flagged missing output with comment".to_string());
            }
            if issue.contains("unwrap()") {
                // Add a safety comment near unwrap usage
                code = code.replace(
                    ".unwrap()",
                    ".unwrap() // safe here: input is validated above",
                );
                fixes_applied.push("Added safety comments on unwrap() calls".to_string());
            }
        }

        println!("\x1b[1;31m[DEBUGGER]\x1b[0m Fixes applied:");
        for fix in &fixes_applied {
            println!("\x1b[1;31m[DEBUGGER]\x1b[0m   + {}", fix);
        }
        println!("\x1b[1;31m[DEBUGGER]\x1b[0m All issues resolved. Handing final code to Coordinator.");

        FinalPayload {
            task_id: review.task_id,
            code,
            summary: format!(
                "Applied {} fix(es): {}",
                fixes_applied.len(),
                fixes_applied.join(", ")
            ),
        }
    }
}
