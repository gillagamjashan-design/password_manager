use crate::messages::{CodePayload, ReviewPayload};

/// The Reviewer agent checks code for quality issues using static analysis rules.
/// Brain: Built-in — applies Rust code quality checks directly with specific diagnostics.
pub struct ReviewerAgent;

impl ReviewerAgent {
    pub fn new() -> Self { ReviewerAgent }

    pub fn process(&self, code_payload: CodePayload) -> ReviewPayload {
        println!("\n\x1b[1;35m[REVIEWER]\x1b[0m Received code for review. Analyzing...");
        println!("\x1b[1;35m[REVIEWER]\x1b[0m \x1b[2m· Brain: Built-in (Security & Docs)\x1b[0m");

        let issues = self.review_code(&code_payload.code);
        let approved = issues.is_empty();

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

    fn review_code(&self, code: &str) -> Vec<String> {
        let mut issues = vec![];

        // Check 1: Missing fn main() entry point
        if !code.contains("fn main()") {
            issues.push("Missing fn main() — code needs an entry point to be runnable".to_string());
        }

        // Check 2: No comments present (neither line comments nor doc comments)
        let has_line_comment = code.contains("//");
        let has_doc_comment  = code.contains("///");
        if !has_line_comment && !has_doc_comment {
            issues.push("No comments found — add // inline comments or /// doc comments to explain the code".to_string());
        }

        // Check 3: Excessive .unwrap() calls (more than 3 is a smell)
        let unwrap_count = code.matches(".unwrap()").count();
        if unwrap_count > 3 {
            issues.push(format!(
                "Found {} .unwrap() calls — more than 3 is risky; use match, if let, or ? for safer error handling",
                unwrap_count
            ));
        }

        // Check 4: Unfinished placeholders
        if code.contains("todo!()") || code.contains("unimplemented!()") {
            let placeholder_count =
                code.matches("todo!()").count() + code.matches("unimplemented!()").count();
            issues.push(format!(
                "Found {} placeholder(s) (todo!() or unimplemented!()) — replace with real implementations",
                placeholder_count
            ));
        }

        // Check 5: Magic numbers without explanation (standalone numeric literals > 9)
        // Look for bare integer literals that are not in comments or string literals
        let has_magic_numbers = Self::detect_magic_numbers(code);
        if has_magic_numbers {
            issues.push(
                "Magic numbers detected — consider naming constants with 'const NAME: Type = value;' for clarity"
                    .to_string(),
            );
        }

        // Check 6: Functions longer than 40 lines (rough complexity check)
        let max_fn_lines = Self::longest_function_lines(code);
        if max_fn_lines > 40 {
            issues.push(format!(
                "A function spans ~{} lines — consider breaking it into smaller helper functions",
                max_fn_lines
            ));
        }

        // Check 7: Mutable variables that could be immutable (basic check: mut declared but no reassignment hint)
        let mut_count = code.matches("let mut ").count();
        let reassign_count = code.matches(" = ").count().saturating_sub(code.matches("let ").count());
        if mut_count > 2 && reassign_count == 0 {
            issues.push(
                "Several 'let mut' declarations found but few reassignments — prefer 'let' when possible"
                    .to_string(),
            );
        }

        issues
    }

    /// Detects magic numbers: bare integer literals > 9 not inside strings or comments.
    /// This is a lightweight heuristic that avoids parsing string literals.
    fn detect_magic_numbers(code: &str) -> bool {
        // Strip out string literals and comments for analysis
        let mut stripped = String::with_capacity(code.len());
        let mut chars = code.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '/' {
                if chars.peek() == Some(&'/') {
                    // Line comment: skip until newline
                    for c in chars.by_ref() {
                        if c == '\n' { break; }
                    }
                    stripped.push('\n');
                    continue;
                }
            }
            if ch == '"' {
                // String literal: skip until closing quote (handle escape sequences)
                stripped.push(' ');
                while let Some(c) = chars.next() {
                    if c == '\\' { chars.next(); } // skip escaped char
                    else if c == '"' { break; }
                }
                continue;
            }
            stripped.push(ch);
        }

        // Now scan for numeric tokens > 9 that appear in code context
        // A magic number is a standalone integer literal not immediately preceded by a letter/underscore
        // (which would make it part of an identifier or suffix like 0u64)
        let mut prev_was_alnum = false;
        let mut in_number = false;
        let mut number_buf = String::new();
        let mut found_magic = false;

        for ch in stripped.chars() {
            if ch.is_ascii_digit() {
                if !prev_was_alnum {
                    in_number = true;
                }
                if in_number {
                    number_buf.push(ch);
                }
                prev_was_alnum = true;
            } else {
                if in_number && !prev_was_alnum {
                    in_number = false;
                    number_buf.clear();
                } else if in_number {
                    // Number ended; check value
                    if let Ok(val) = number_buf.parse::<u64>() {
                        if val > 9 {
                            found_magic = true;
                        }
                    }
                    in_number = false;
                    number_buf.clear();
                }
                prev_was_alnum = ch.is_alphanumeric() || ch == '_';
            }
        }
        // Handle number at end of string
        if in_number {
            if let Ok(val) = number_buf.parse::<u64>() {
                if val > 9 { found_magic = true; }
            }
        }

        found_magic
    }

    /// Returns the line count of the longest function body in the code.
    fn longest_function_lines(code: &str) -> usize {
        let mut max_lines = 0usize;
        let mut depth = 0i32;
        let mut fn_start_line: Option<usize> = None;
        let mut current_line = 0usize;

        for ch in code.chars() {
            if ch == '\n' { current_line += 1; }
            if ch == '{' {
                depth += 1;
                if depth == 1 && fn_start_line.is_none() {
                    fn_start_line = Some(current_line);
                }
            } else if ch == '}' {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = fn_start_line {
                        let len = current_line.saturating_sub(start);
                        if len > max_lines { max_lines = len; }
                    }
                    fn_start_line = None;
                }
            }
        }
        max_lines
    }
}
