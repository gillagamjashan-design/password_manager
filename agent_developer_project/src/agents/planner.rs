use crate::messages::{PlanPayload, TaskPayload};

/// The Planner agent breaks a task description into ordered steps.
/// Think of this as a senior engineer outlining the approach before writing any code.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self {
        PlannerAgent
    }

    /// Takes a task and produces a list of implementation steps.
    pub fn process(&self, task: TaskPayload) -> PlanPayload {
        println!("\n[PLANNER] Received task: \"{}\"", task.description);
        println!("[PLANNER] Breaking task down into steps...");

        let steps = self.generate_steps(&task.description);

        for (i, step) in steps.iter().enumerate() {
            println!("[PLANNER]   Step {}: {}", i + 1, step);
        }

        println!("[PLANNER] Plan complete. Handing off to Coder.");

        PlanPayload {
            task_id: task.task_id,
            steps,
        }
    }

    /// Generate implementation steps based on what the task description contains.
    /// This uses simple keyword matching to simulate planning intelligence.
    fn generate_steps(&self, description: &str) -> Vec<String> {
        let desc = description.to_lowercase();

        // Base steps that apply to every task
        let mut steps = vec![
            "Define the function signature with clear parameter names".to_string(),
            "Write the function body with correct logic".to_string(),
            "Add a comment explaining what the function does".to_string(),
            "Write a main() function that calls and tests the function".to_string(),
            "Make sure the output is printed clearly".to_string(),
        ];

        // Add extra steps based on task keywords
        if desc.contains("prime") || desc.contains("number") {
            steps.insert(1, "Handle edge cases: numbers <= 1 are not prime".to_string());
        }
        if desc.contains("sort") || desc.contains("array") || desc.contains("list") {
            steps.insert(1, "Handle empty input gracefully".to_string());
        }
        if desc.contains("string") || desc.contains("text") {
            steps.insert(1, "Handle empty string input".to_string());
        }

        steps
    }
}
