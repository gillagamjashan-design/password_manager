use crate::messages::{PlanPayload, TaskPayload};

/// The Planner agent breaks a task description into ordered steps.
/// Steps now include task-specific keywords so the Coder can match them.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self { PlannerAgent }

    pub fn process(&self, task: TaskPayload) -> PlanPayload {
        println!("\n\x1b[1;36m[PLANNER]\x1b[0m Received task: \"{}\"", task.description);
        println!("\x1b[1;36m[PLANNER]\x1b[0m Breaking task down into steps...");

        let steps = self.generate_steps(&task.description);

        for (i, step) in steps.iter().enumerate() {
            println!("\x1b[1;36m[PLANNER]\x1b[0m   Step {}: {}", i + 1, step);
        }
        println!("\x1b[1;36m[PLANNER]\x1b[0m Plan complete. Handing off to Coder.");

        PlanPayload { task_id: task.task_id, steps }
    }

    fn generate_steps(&self, description: &str) -> Vec<String> {
        let desc = description.to_lowercase();

        // Extract key topic words to include in steps so the Coder gets better signal
        let topic = self.extract_topic(&desc);

        let mut steps = vec![
            format!("Define a function signature for: {}", topic),
            format!("Implement the core logic to handle: {}", topic),
            "Add a comment explaining how the function works".to_string(),
            "Write a main() function that calls the function with test cases".to_string(),
            "Print results clearly so users can verify the output".to_string(),
        ];

        // Add task-specific extra steps based on keywords
        if desc.contains("prime") || desc.contains("number") {
            steps.insert(1, "Handle edge cases: numbers <= 1, even numbers".to_string());
        }
        if desc.contains("sort") || desc.contains("order") {
            steps.insert(1, "Handle empty input and single-element inputs".to_string());
        }
        if desc.contains("string") || desc.contains("text") || desc.contains("revers") {
            steps.insert(1, "Handle empty string input".to_string());
        }
        if desc.contains("search") {
            steps.insert(1, "Handle not-found case (return None or -1)".to_string());
        }
        if desc.contains("file") {
            steps.insert(1, "Handle file I/O errors with Result".to_string());
        }

        steps
    }

    /// Extracts the most meaningful 1-4 words from the task to use in step descriptions.
    fn extract_topic(&self, description: &str) -> String {
        let stop_words = ["a", "an", "the", "that", "this", "write", "create",
            "make", "build", "implement", "function", "in", "for", "which"];
        let words: Vec<&str> = description
            .split_whitespace()
            .filter(|w| !stop_words.contains(w) && w.len() >= 3)
            .take(4)
            .collect();
        if words.is_empty() { description.to_string() } else { words.join(" ") }
    }
}
