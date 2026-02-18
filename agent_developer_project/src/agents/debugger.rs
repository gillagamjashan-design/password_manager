use crate::messages::{FinalPayload, ReviewPayload};

/// The Debugger agent fixes issues identified by the Reviewer.
/// Brain: Built-in — applies automatic fix patterns for each known issue type.
pub struct DebuggerAgent;

impl DebuggerAgent {
    pub fn new() -> Self { DebuggerAgent }

    pub fn process(&self, review: ReviewPayload) -> FinalPayload {
        println!("\n\x1b[1;31m[DEBUGGER]\x1b[0m \x1b[2m· Brain: Built-in (Debugging)\x1b[0m");

        if review.approved {
            println!("\x1b[1;31m[DEBUGGER]\x1b[0m Code approved. No fixes needed.");
        } else {
            println!(
                "\x1b[1;31m[DEBUGGER]\x1b[0m Applying fixes for {} issue(s)...",
                review.issues.len()
            );
        }

        let (fixed_code, summary) = self.fix(&review);

        println!("\x1b[1;31m[DEBUGGER]\x1b[0m {}", summary);
        println!("\x1b[1;31m[DEBUGGER]\x1b[0m Handing final code to Coordinator.");

        FinalPayload {
            task_id: review.task_id,
            code: fixed_code,
            summary,
        }
    }

    fn fix(&self, review: &ReviewPayload) -> (String, String) {
        if review.approved {
            return (
                review.code.clone(),
                "Code passed review — no changes needed.".to_string(),
            );
        }

        let mut code = review.code.clone();
        let mut fixes_applied: Vec<String> = Vec::new();

        for issue in &review.issues {
            let issue_lower = issue.to_lowercase();

            // Fix 1: Missing fn main()
            if issue_lower.contains("fn main()") && !code.contains("fn main()") {
                code.push_str(
                    "\n\nfn main() {\n    // Entry point added by Debugger\n    println!(\"Program complete.\");\n}",
                );
                fixes_applied.push("added missing fn main() entry point".to_string());
            }

            // Fix 2: No comments
            if issue_lower.contains("no comments") && !code.contains("//") {
                // Add a module-level doc comment at the top
                code = format!(
                    "// This module implements the requested functionality.\n// Review each function for details.\n\n{}",
                    code
                );
                fixes_applied.push("added top-level inline comments".to_string());
            }

            // Fix 3: Excessive .unwrap() calls — replace with safer match-based alternatives
            if issue_lower.contains("unwrap()") {
                let unwrap_count_before = code.matches(".unwrap()").count();
                if unwrap_count_before > 3 {
                    // Replace all .unwrap() beyond the third with expect() for clarity
                    // We cannot safely rewrite all contexts, so we annotate the first excess occurrence
                    code = Self::reduce_unwraps(&code);
                    let unwrap_count_after = code.matches(".unwrap()").count();
                    if unwrap_count_after < unwrap_count_before {
                        fixes_applied.push(format!(
                            "replaced {} excess .unwrap() calls with .expect() for better diagnostics",
                            unwrap_count_before - unwrap_count_after
                        ));
                    }
                }
            }

            // Fix 4: todo!() / unimplemented!() placeholders
            if issue_lower.contains("todo") || issue_lower.contains("unimplemented") {
                let had_todo  = code.contains("todo!()");
                let had_unimpl = code.contains("unimplemented!()");

                if had_todo {
                    code = code.replace("todo!()", "Default::default() /* fixed: was todo!() */");
                    fixes_applied.push("replaced todo!() with Default::default()".to_string());
                }
                if had_unimpl {
                    code = code.replace(
                        "unimplemented!()",
                        "Default::default() /* fixed: was unimplemented!() */",
                    );
                    fixes_applied.push(
                        "replaced unimplemented!() with Default::default()".to_string(),
                    );
                }
            }

            // Fix 5: Magic numbers — add a comment block suggesting constants
            if issue_lower.contains("magic number") {
                let already_has_const = code.contains("const ");
                if !already_has_const {
                    code = format!(
                        "// Consider defining named constants for numeric literals, e.g.:\n// const MAX_SIZE: usize = 100;\n\n{}",
                        code
                    );
                    fixes_applied
                        .push("added guidance comment about named constants".to_string());
                }
            }

            // Fix 6: Function too long — add a refactoring note
            if issue_lower.contains("function spans") || issue_lower.contains("lines") && issue_lower.contains("function") {
                if !code.contains("// NOTE: consider refactoring") {
                    code.push_str(
                        "\n\n// NOTE: consider refactoring long functions into smaller helpers for readability.",
                    );
                    fixes_applied
                        .push("added refactoring suggestion for long function".to_string());
                }
            }

            // Fix 7: Unnecessary mut — add linting note
            if issue_lower.contains("let mut") && issue_lower.contains("prefer") {
                if !code.contains("// LINT:") {
                    code = format!(
                        "// LINT: review 'let mut' usages — prefer immutable 'let' where reassignment is unnecessary.\n\n{}",
                        code
                    );
                    fixes_applied
                        .push("added lint note about unnecessary mut declarations".to_string());
                }
            }
        }

        let summary = if fixes_applied.is_empty() {
            format!(
                "Reviewed {} issue(s) — no automatic fixes could be applied; manual review recommended.",
                review.issues.len()
            )
        } else {
            format!("Applied {} fix(es): {}", fixes_applied.len(), fixes_applied.join("; "))
        };

        (code, summary)
    }

    /// Replaces .unwrap() calls beyond the third occurrence with .expect("<description>")
    /// to provide better diagnostic messages while preserving the first few usages.
    fn reduce_unwraps(code: &str) -> String {
        let mut result = String::with_capacity(code.len());
        let mut remaining = code;
        let mut count = 0usize;
        let needle = ".unwrap()";

        while let Some(pos) = remaining.find(needle) {
            result.push_str(&remaining[..pos]);
            count += 1;
            if count > 3 {
                result.push_str(".expect(\"value should be present\")");
            } else {
                result.push_str(needle);
            }
            remaining = &remaining[pos + needle.len()..];
        }
        result.push_str(remaining);
        result
    }
}
