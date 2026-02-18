use crate::ai_client::call_claude;
use crate::messages::{PlanPayload, TaskPayload};

/// The Planner agent breaks a task into ordered implementation steps.
/// Powered by: Claude (Anthropic) — Architecture specialist.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self { PlannerAgent }

    pub fn process(&self, task: TaskPayload) -> PlanPayload {
        println!("\n\x1b[1;36m[PLANNER]\x1b[0m Received task: \"{}\"", task.description);
        println!("\x1b[1;36m[PLANNER]\x1b[0m \x1b[2m· Brain: Claude (Architecture)\x1b[0m");
        println!("\x1b[1;36m[PLANNER]\x1b[0m Breaking task down into steps...");

        let steps = self.generate_steps(&task.description);

        for (i, step) in steps.iter().enumerate() {
            println!("\x1b[1;36m[PLANNER]\x1b[0m   Step {}: {}", i + 1, step);
        }
        println!("\x1b[1;36m[PLANNER]\x1b[0m Plan complete. Handing off to Coder.");

        PlanPayload { task_id: task.task_id, steps }
    }

    fn generate_steps(&self, description: &str) -> Vec<String> {
        let system = "You are a software architecture specialist. \
            Your job is to break down coding tasks into clear, ordered implementation steps. \
            Return ONLY a numbered list of steps (1. step one 2. step two etc). \
            Each step should be one concise sentence. Aim for 4-6 steps. \
            Focus on Rust implementation specifics.";

        let user = format!("Break this coding task into implementation steps: {description}");

        match call_claude(system, &user) {
            Ok(response) => self.parse_steps(&response),
            Err(e) => {
                println!("\x1b[1;36m[PLANNER]\x1b[0m \x1b[33mClaude unavailable: {e}\x1b[0m");
                println!("\x1b[1;36m[PLANNER]\x1b[0m \x1b[33mUsing fallback planning.\x1b[0m");
                self.fallback_steps(description)
            }
        }
    }

    /// Parses a numbered list response into a Vec of step strings.
    fn parse_steps(&self, response: &str) -> Vec<String> {
        response
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                // Match lines starting with a number and period/dot: "1. ..." or "1) ..."
                if trimmed.len() > 2 {
                    let rest = trimmed
                        .trim_start_matches(|c: char| c.is_numeric())
                        .trim_start_matches(['.', ')', ' ']);
                    if !rest.is_empty() && rest != trimmed {
                        return Some(rest.to_string());
                    }
                }
                None
            })
            .filter(|s| !s.is_empty())
            .take(8)
            .collect()
    }

    fn fallback_steps(&self, description: &str) -> Vec<String> {
        vec![
            format!("Define the function signature for: {description}"),
            "Implement the core logic".to_string(),
            "Add comments explaining how the function works".to_string(),
            "Write a main() function with test cases".to_string(),
            "Print results so users can verify output".to_string(),
        ]
    }
}
