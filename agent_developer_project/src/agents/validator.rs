use crate::messages::{FinalPayload, ValidationPayload};

/// The Validator agent checks whether the code addresses the user's task.
/// Brain: Built-in — extracts meaningful keywords, checks code structure and relevance.
pub struct ValidatorAgent;

impl ValidatorAgent {
    pub fn new() -> Self { ValidatorAgent }

    pub fn process(&self, result: &FinalPayload, task_description: &str) -> ValidationPayload {
        println!("\n\x1b[1;33m[VALIDATOR]\x1b[0m Checking output matches task...");
        println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[2m· Brain: Built-in (Testing)\x1b[0m");

        let (passed, reason) = self.validate(&result.code, task_description);

        if passed {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mValidation passed: {reason}\x1b[0m");
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[31mValidation failed: {reason}\x1b[0m");
        }

        ValidationPayload { task_id: result.task_id, passed, reason }
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
