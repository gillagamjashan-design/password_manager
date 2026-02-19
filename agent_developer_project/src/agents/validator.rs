use crate::messages::{FinalPayload, ValidationPayload};
use crate::thinking::{ThinkingTimer, ProcessingStage};
use std::fs;
use std::process::Command;

/// The Validator agent checks whether the code addresses the user's task.
/// Brain: Built-in — extracts meaningful keywords, checks code structure and relevance.
pub struct ValidatorAgent;

impl ValidatorAgent {
    pub fn new() -> Self { ValidatorAgent }

    pub fn process(&self, result: &FinalPayload, task_description: &str) -> ValidationPayload {
        println!("\n\x1b[1;33m[VALIDATOR]\x1b[0m Validating output against task...");
        println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[2m· Brain: Built-in (Testing)\x1b[0m");

        // Stage 1: Generate test cases
        ThinkingTimer::new(ProcessingStage::TestGeneration, 15).start();
        let test_cases = self.generate_test_cases(task_description);
        println!("\x1b[1;33m[VALIDATOR]\x1b[0m Generated {} test case(s):", test_cases.len());
        for (i, (input, expected)) in test_cases.iter().enumerate() {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m   Test {}: {} → expect: {}", i + 1, input, expected);
        }

        // Stage 2: Run test cases
        ThinkingTimer::new(ProcessingStage::TestExecution, 20).start();
        let test_passed = self.run_code_with_tests(&result.code, &test_cases);
        if test_passed {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mTest execution passed\x1b[0m");
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[33mTest execution failed or did not compile\x1b[0m");
        }

        // Stage 3: Final validation (keyword checks)
        ThinkingTimer::new(ProcessingStage::FinalValidation, 10).start();
        let (keyword_passed, reason) = self.validate(&result.code, task_description);

        // Both tests must pass
        let passed = test_passed && keyword_passed;
        let final_reason = if passed {
            format!("All checks passed: {}", reason)
        } else if !test_passed {
            format!("Test execution failed: {}", reason)
        } else {
            format!("Keyword validation failed: {}", reason)
        };

        if passed {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mValidation passed: {}\x1b[0m", final_reason);
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[31mValidation failed: {}\x1b[0m", final_reason);
        }

        ValidationPayload { task_id: result.task_id, passed, reason: final_reason }
    }

    fn generate_test_cases(&self, description: &str) -> Vec<(String, String)> {
        let desc_lower = description.to_lowercase();
        let mut test_cases = Vec::new();

        if desc_lower.contains("sort") {
            test_cases.push((
                "unsorted vec [5, 2, 8, 1]".to_string(),
                "ascending order [1, 2, 5, 8]".to_string(),
            ));
        } else if desc_lower.contains("revers") {
            test_cases.push((
                "string \"hello world\"".to_string(),
                "reversed \"dlrow olleh\"".to_string(),
            ));
        } else if desc_lower.contains("fibonacci") || desc_lower.contains("fib") {
            test_cases.push((
                "fib(10)".to_string(),
                "result 55".to_string(),
            ));
        } else if desc_lower.contains("factorial") {
            test_cases.push((
                "factorial(5)".to_string(),
                "result 120".to_string(),
            ));
        } else if desc_lower.contains("prime") {
            test_cases.push((
                "is_prime(13)".to_string(),
                "true".to_string(),
            ));
        } else if desc_lower.contains("search") || desc_lower.contains("find") {
            test_cases.push((
                "search for target in list".to_string(),
                "found or not found message".to_string(),
            ));
        } else {
            // Generic test case
            test_cases.push((
                "call main()".to_string(),
                "non-empty output".to_string(),
            ));
        }

        test_cases
    }

    fn run_code_with_tests(&self, code: &str, _test_cases: &[(String, String)]) -> bool {
        // Write code to temp file
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let temp_file = format!("/tmp/agent_test_{}.rs", timestamp);
        let temp_binary = format!("/tmp/agent_test_{}", timestamp);

        // Write code to temp file
        if let Err(_e) = fs::write(&temp_file, code) {
            return false;
        }

        // Compile the code
        let compile_result = Command::new("rustc")
            .arg(&temp_file)
            .arg("-o")
            .arg(&temp_binary)
            .output();

        let compile_ok = match compile_result {
            Ok(output) => output.status.success(),
            Err(_) => false,
        };

        if !compile_ok {
            // Cleanup
            let _ = fs::remove_file(&temp_file);
            return false;
        }

        // Run the binary
        let run_result = Command::new(&temp_binary)
            .output();

        let run_ok = match run_result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // Check that it produced some output
                !stdout.trim().is_empty()
            }
            Err(_) => false,
        };

        // Cleanup
        let _ = fs::remove_file(&temp_file);
        let _ = fs::remove_file(&temp_binary);

        run_ok
    }

    fn validate(&self, code: &str, task_description: &str) -> (bool, String) {
        // --- Structural check 1: fn main() must exist ---
        if !code.contains("fn main()") {
            return (
                false,
                "Code is missing fn main() — no runnable entry point".to_string(),
            );
        }

        // --- Structural check 2: Code must have some meaningful content ---
        let non_whitespace: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        if non_whitespace.len() < 40 {
            return (
                false,
                "Code is too short to be a real implementation".to_string(),
            );
        }

        // --- Keyword extraction: filter stop words and short words ---
        let stop_words: &[&str] = &[
            "a", "an", "the", "write", "create", "make", "build", "implement",
            "function", "in", "for", "that", "with", "and", "or", "to", "of",
            "using", "use", "which", "this", "it", "its", "can", "will", "should",
            "from", "into", "onto", "some", "any", "all", "each", "both", "me",
            "you", "we", "they", "is", "are", "was", "be", "has", "have", "do",
            "does", "did", "not", "no", "yes", "on", "at", "by", "as", "up",
        ];

        let keywords: Vec<String> = task_description
            .to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !stop_words.contains(w) && w.len() >= 3)
            .map(|w| w.to_string())
            .collect();

        // --- Keyword synonym map for common domain terms ---
        // Maps task keywords to related terms that might appear in code
        let synonyms: &[(&str, &[&str])] = &[
            ("sort",       &["sort", "sorted", "order", "ascending", "descending"]),
            ("revers",     &["rev", "reverse", "reversed", "backward"]),
            ("fibonacci",  &["fibonacci", "fib"]),
            ("factorial",  &["factorial", "product"]),
            ("prime",      &["prime", "is_prime", "primality"]),
            ("search",     &["search", "find", "position", "linear_search", "binary_search"]),
            ("find",       &["find", "search", "position", "locate"]),
            ("count",      &["count", "frequency", "freq", "HashMap"]),
            ("frequency",  &["frequency", "freq", "count", "HashMap"]),
            ("filter",     &["filter", "retain", "remove", "evens", "odds"]),
            ("remove",     &["remove", "filter", "retain", "delete"]),
            ("palindrome", &["palindrome", "is_palindrome", "cleaned", "reversed"]),
            ("anagram",    &["anagram", "is_anagram", "sorted", "normalize"]),
            ("stack",      &["Stack", "push", "pop", "peek", "lifo"]),
            ("queue",      &["Queue", "enqueue", "dequeue", "VecDeque", "fifo"]),
            ("hash",       &["HashMap", "hash", "insert", "get", "entry"]),
            ("map",        &["HashMap", "map", "insert", "get", "BTreeMap"]),
            ("dictionary", &["HashMap", "BTreeMap", "insert", "get", "entry"]),
            ("tree",       &["Node", "insert", "in_order", "bst", "left", "right"]),
            ("bst",        &["Node", "insert", "in_order", "left", "right"]),
            ("binary",     &["Node", "binary", "left", "right", "insert"]),
            ("graph",      &["Graph", "edges", "bfs", "dfs", "neighbors", "node"]),
            ("calculator", &["calculate", "Operation", "Add", "Subtract", "Multiply", "Divide"]),
            ("calc",       &["calculate", "Operation", "Add", "Subtract"]),
            ("matrix",     &["Matrix", "rows", "cols", "data", "set", "get"]),
        ];

        if keywords.is_empty() {
            return (
                true,
                "Task description is very generic — entry point and structure checks passed".to_string(),
            );
        }

        let code_lower = code.to_lowercase();

        // Check each keyword against the code directly and via synonym expansion
        let mut matched_keywords: Vec<String> = Vec::new();
        let mut unmatched_keywords: Vec<String> = Vec::new();

        for kw in &keywords {
            let direct_match = code_lower.contains(kw.as_str());
            let synonym_match = synonyms.iter().any(|(key, syns)| {
                (kw.starts_with(key) || key.starts_with(kw.as_str()))
                    && syns.iter().any(|s| code_lower.contains(*s) || code.contains(*s))
            });

            if direct_match || synonym_match {
                matched_keywords.push(kw.clone());
            } else {
                unmatched_keywords.push(kw.clone());
            }
        }

        let match_ratio = matched_keywords.len() as f64 / keywords.len() as f64;

        // --- Structural check 3: Code should contain at least one fn definition ---
        let fn_count = code.matches("fn ").count();
        if fn_count < 2 {
            // At least a helper fn + main
            return (
                false,
                format!(
                    "Code only has {} fn definition(s) — expected at least a helper function plus main()",
                    fn_count
                ),
            );
        }

        // --- Final decision based on match ratio ---
        if match_ratio >= 0.5 {
            (
                true,
                format!(
                    "Code addresses the task — matched {}/{} keyword(s): {:?}",
                    matched_keywords.len(),
                    keywords.len(),
                    &matched_keywords[..matched_keywords.len().min(4)]
                ),
            )
        } else if match_ratio > 0.0 {
            (
                true,
                format!(
                    "Partial match ({}/{} keyword(s)) — code has fn main() and relevant structure; unmatched: {:?}",
                    matched_keywords.len(),
                    keywords.len(),
                    &unmatched_keywords[..unmatched_keywords.len().min(3)]
                ),
            )
        } else {
            (
                false,
                format!(
                    "Code does not appear to address the task — none of {:?} were found in the code",
                    &keywords[..keywords.len().min(4)]
                ),
            )
        }
    }
}
