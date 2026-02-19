use crate::messages::{PlanPayload, TaskPayload};
use crate::thinking::{ThinkingTimer, ProcessingStage};

/// The Planner agent breaks a task into ordered implementation steps.
/// Brain: Built-in — keyword-based task analysis covering 15+ task types.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self { PlannerAgent }

    pub fn process(&self, task: TaskPayload) -> PlanPayload {
        println!("\n\x1b[1;36m[PLANNER]\x1b[0m Received task: \"{}\"", task.description);
        println!("\x1b[1;36m[PLANNER]\x1b[0m \x1b[2m· Brain: Built-in (Architecture)\x1b[0m");

        // Stage 1: Extract requirements
        ThinkingTimer::new(ProcessingStage::RequirementsExtraction, 10).start();
        let requirements = self.extract_requirements(&task.description);
        println!("\x1b[1;36m[PLANNER]\x1b[0m Extracted requirements:");
        for (i, req) in requirements.iter().enumerate() {
            println!("\x1b[1;36m[PLANNER]\x1b[0m   {}. {}", i + 1, req);
        }

        // Stage 2: Generate implementation steps
        ThinkingTimer::new(ProcessingStage::Planning, 15).start();
        println!("\x1b[1;36m[PLANNER]\x1b[0m Breaking task down into steps...");
        let mut steps = self.generate_steps(&task.description);

        // Prepend requirements as special entries
        let mut requirements_steps: Vec<String> = requirements
            .iter()
            .map(|req| format!("REQUIREMENT: {}", req))
            .collect();
        requirements_steps.append(&mut steps);

        for (i, step) in requirements_steps.iter().enumerate() {
            println!("\x1b[1;36m[PLANNER]\x1b[0m   Step {}: {}", i + 1, step);
        }
        println!("\x1b[1;36m[PLANNER]\x1b[0m Plan complete. Handing off to Coder.");

        PlanPayload { task_id: task.task_id, steps: requirements_steps }
    }

    fn extract_requirements(&self, description: &str) -> Vec<String> {
        let mut requirements = Vec::new();
        let desc_lower = description.to_lowercase();
        let words: Vec<&str> = description.split_whitespace().collect();

        // Extract input requirements
        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();
            if word_lower == "takes" || word_lower == "accepts" || word_lower == "given" || word_lower == "input" {
                if i + 1 < words.len() {
                    let input_desc = words[i + 1..].iter().take(5).cloned().collect::<Vec<_>>().join(" ");
                    requirements.push(format!("Input: {}", input_desc));
                    break;
                }
            }
        }

        // Extract output requirements
        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();
            if word_lower == "returns" || word_lower == "produces" || word_lower == "output" {
                if i + 1 < words.len() {
                    let output_desc = words[i + 1..].iter().take(5).cloned().collect::<Vec<_>>().join(" ");
                    requirements.push(format!("Output: {}", output_desc));
                    break;
                }
            }
        }

        // Extract constraints
        if desc_lower.contains("must be o(") || desc_lower.contains("complexity") {
            requirements.push("Performance constraint specified in task".to_string());
        }
        if desc_lower.contains("recursion") || desc_lower.contains("recursive") {
            requirements.push("Must use recursion".to_string());
        }
        if desc_lower.contains("no unwrap") || desc_lower.contains("error handling") {
            requirements.push("Must handle errors properly without unwrap()".to_string());
        }

        // Extract edge cases
        if desc_lower.contains("empty") {
            requirements.push("Handle empty input".to_string());
        }
        if desc_lower.contains("negative") {
            requirements.push("Handle negative numbers".to_string());
        }
        if desc_lower.contains("unicode") {
            requirements.push("Support Unicode characters".to_string());
        }

        // Default requirements if none found
        if requirements.is_empty() {
            requirements.push("Process the input meaningfully".to_string());
            requirements.push("Return a valid result".to_string());
            requirements.push("Handle edge cases (empty/null inputs)".to_string());
        }

        requirements
    }

    fn generate_steps(&self, description: &str) -> Vec<String> {
        let desc = description.to_lowercase();

        if desc.contains("sort") || desc.contains("order") {
            vec![
                "Define a function that takes a Vec<i32> as input".to_string(),
                "Use Rust's built-in .sort() method for ascending order".to_string(),
                "Handle edge cases: empty slice and single-element slice".to_string(),
                "Add /// doc comments explaining the sort logic and complexity O(n log n)".to_string(),
                "Write a main() that tests sorting with a sample vec and prints before/after".to_string(),
            ]
        } else if desc.contains("revers") {
            vec![
                "Define a function that takes a &str or Vec as input".to_string(),
                "Use Rust's .chars().rev().collect() to reverse the string".to_string(),
                "Return the reversed value as a new owned String".to_string(),
                "Add /// doc comments explaining the reversal approach".to_string(),
                "Write a main() that tests multiple example inputs and prints the results".to_string(),
            ]
        } else if desc.contains("fibonacci") || desc.contains("fib") {
            vec![
                "Define a function fn fibonacci(n: u64) -> u64".to_string(),
                "Implement iteratively using two accumulators to avoid stack overflow".to_string(),
                "Handle base cases explicitly: fib(0)=0, fib(1)=1".to_string(),
                "Add /// doc comments explaining the iterative approach and O(n) complexity".to_string(),
                "Write a main() that prints the first 10 Fibonacci numbers in a loop".to_string(),
            ]
        } else if desc.contains("factorial") {
            vec![
                "Define a function fn factorial(n: u64) -> u64".to_string(),
                "Implement iteratively using a running product over 1..=n".to_string(),
                "Handle the edge case: 0! = 1 (empty product convention)".to_string(),
                "Add /// doc comments explaining the factorial definition".to_string(),
                "Write a main() that prints factorials for n = 0 through 12".to_string(),
            ]
        } else if desc.contains("prime") {
            vec![
                "Define a function fn is_prime(n: u64) -> bool".to_string(),
                "Implement trial division up to sqrt(n) for efficiency".to_string(),
                "Handle edge cases: n < 2 is not prime, n == 2 is prime, skip even numbers".to_string(),
                "Add /// doc comments explaining trial division and O(sqrt n) complexity".to_string(),
                "Write a main() that collects and prints all prime numbers up to 50".to_string(),
            ]
        } else if desc.contains("search") || desc.contains("find") {
            vec![
                "Define a function fn linear_search(data: &[i32], target: i32) -> Option<usize>".to_string(),
                "Iterate through the slice using .iter().position() for clean code".to_string(),
                "Return Some(index) when found, None when absent".to_string(),
                "Add /// doc comments explaining linear search and O(n) time complexity".to_string(),
                "Write a main() that demonstrates both found and not-found cases".to_string(),
            ]
        } else if desc.contains("count") || desc.contains("frequency") {
            vec![
                "Define a function that takes a &[&str] and returns a frequency map".to_string(),
                "Use HashMap<&str, usize> to accumulate counts with .entry().or_insert(0)".to_string(),
                "Iterate through all items and increment the corresponding count".to_string(),
                "Add /// doc comments explaining the counting and HashMap usage".to_string(),
                "Write a main() that counts word frequencies and prints sorted results".to_string(),
            ]
        } else if desc.contains("filter") || desc.contains("remove") {
            vec![
                "Define a function that takes a Vec<i32> and returns a filtered Vec<i32>".to_string(),
                "Use .into_iter().filter() with a closure for the predicate logic".to_string(),
                "Collect results into a new Vec with .collect()".to_string(),
                "Add /// doc comments explaining what the filter retains or removes".to_string(),
                "Write a main() that shows input vs filtered output side by side".to_string(),
            ]
        } else if desc.contains("palindrome") {
            vec![
                "Define a function fn is_palindrome(s: &str) -> bool".to_string(),
                "Normalize the input: lowercase and remove non-alphabetic characters".to_string(),
                "Compare the cleaned string against its reverse using .chars().rev()".to_string(),
                "Add /// doc comments explaining the palindrome definition and normalization".to_string(),
                "Write a main() that tests several words including edge cases like empty string".to_string(),
            ]
        } else if desc.contains("anagram") {
            vec![
                "Define a function fn is_anagram(a: &str, b: &str) -> bool".to_string(),
                "Sort the characters of both strings after lowercasing them".to_string(),
                "Compare the two sorted character vectors for equality".to_string(),
                "Add /// doc comments explaining the anagram definition and sort-based approach".to_string(),
                "Write a main() that tests several pairs and prints true/false results".to_string(),
            ]
        } else if desc.contains("stack") {
            vec![
                "Define a struct Stack<T> with an inner Vec<T> field".to_string(),
                "Implement push(), pop() -> Option<T>, peek() -> Option<&T>, and is_empty()".to_string(),
                "Ensure push appends to the end and pop removes from the end (LIFO order)".to_string(),
                "Add /// doc comments on each method explaining the LIFO semantics".to_string(),
                "Write a main() that pushes several values, peeks, pops all, and prints each step".to_string(),
            ]
        } else if desc.contains("queue") {
            vec![
                "Define a struct Queue<T> with an inner VecDeque<T> field".to_string(),
                "Implement enqueue(), dequeue() -> Option<T>, peek() -> Option<&T>, and is_empty()".to_string(),
                "Ensure enqueue appends to the back and dequeue removes from the front (FIFO order)".to_string(),
                "Add /// doc comments on each method explaining the FIFO semantics".to_string(),
                "Write a main() that enqueues several values, peeks, dequeues all, and prints each step".to_string(),
            ]
        } else if desc.contains("hash") || desc.contains("map") || desc.contains("dictionary") {
            vec![
                "Import std::collections::HashMap for the implementation".to_string(),
                "Define a function that inserts, looks up, and removes key-value pairs".to_string(),
                "Demonstrate HashMap::new(), .insert(), .get(), .remove(), and .contains_key()".to_string(),
                "Add /// doc comments explaining HashMap usage and O(1) average-case operations".to_string(),
                "Write a main() that builds a small phone-book style map and queries it".to_string(),
            ]
        } else if desc.contains("tree") || desc.contains("bst") || desc.contains("binary") {
            vec![
                "Define a Node struct with value: i32, left: Option<Box<Node>>, right: Option<Box<Node>>".to_string(),
                "Implement an insert() method that places values correctly in BST order".to_string(),
                "Implement an in_order() method that returns a sorted Vec<i32> via DFS".to_string(),
                "Add /// doc comments explaining BST invariants and tree traversal".to_string(),
                "Write a main() that inserts several values and prints the in-order traversal".to_string(),
            ]
        } else if desc.contains("graph") {
            vec![
                "Define a Graph struct with an adjacency list: HashMap<usize, Vec<usize>>".to_string(),
                "Implement add_node(), add_edge(from, to), and neighbors(node) methods".to_string(),
                "Implement a BFS or DFS traversal that returns visited nodes in order".to_string(),
                "Add /// doc comments explaining graph representation and traversal algorithm".to_string(),
                "Write a main() that builds a small graph and prints the traversal result".to_string(),
            ]
        } else if desc.contains("calculator") || desc.contains("calc") {
            vec![
                "Define an enum Operation with Add, Subtract, Multiply, Divide variants".to_string(),
                "Define a function fn calculate(a: f64, op: Operation, b: f64) -> Result<f64, String>".to_string(),
                "Handle division by zero by returning Err with a descriptive message".to_string(),
                "Add /// doc comments explaining each operation and error handling".to_string(),
                "Write a main() that demonstrates all four operations and prints results".to_string(),
            ]
        } else if desc.contains("matrix") {
            vec![
                "Define a Matrix struct with rows: usize, cols: usize, and data: Vec<Vec<f64>>".to_string(),
                "Implement Matrix::new(), get(r, c), set(r, c, val), and add(&Matrix) methods".to_string(),
                "Validate dimensions in add() and return an error string if they mismatch".to_string(),
                "Add /// doc comments explaining matrix operations and indexing".to_string(),
                "Write a main() that creates two 2x2 matrices, adds them, and prints the result".to_string(),
            ]
        } else {
            // Intelligently derive steps from the description words
            let meaningful_words: Vec<&str> = description
                .split_whitespace()
                .filter(|w| w.len() > 4 && w.chars().all(|c| c.is_alphabetic()))
                .collect();
            let noun = meaningful_words.first().copied().unwrap_or("task");
            let verb = meaningful_words.get(1).copied().unwrap_or("process");
            vec![
                format!("Define the function signature for: {description}"),
                format!("Implement the core logic to {} the {}", verb, noun),
                "Handle edge cases: empty input, invalid values, and boundary conditions".to_string(),
                "Add /// doc comments explaining inputs, outputs, and the algorithm used".to_string(),
                "Write a main() function with multiple test cases and print all results".to_string(),
            ]
        }
    }
}
