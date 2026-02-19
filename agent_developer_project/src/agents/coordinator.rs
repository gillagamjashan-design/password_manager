use crate::messages::{FinalPayload, TaskPayload};
use crate::task::{Task, TaskStatus};
use crate::file_writer;
use std::collections::HashMap;

/// The Coordinator agent manages the entire pipeline.
/// It assigns tasks to the team and receives the final result.
/// Think of this as the team lead or project manager.
pub struct CoordinatorAgent {
    task_counter: u32,
    task_descriptions: HashMap<u32, String>,
}

impl CoordinatorAgent {
    pub fn new() -> Self {
        CoordinatorAgent {
            task_counter: 0,
            task_descriptions: HashMap::new(),
        }
    }

    /// Creates a new task and dispatches it to the pipeline.
    pub fn assign_task(&mut self, description: &str) -> (Task, TaskPayload) {
        self.task_counter += 1;
        let id = self.task_counter;

        // Store task description for file writing later
        self.task_descriptions.insert(id, description.to_string());

        println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m New task assigned.");
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m   ID: #{}", id);
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m   Task: \"{}\"", description);
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m Dispatching to Planner...");

        let task = Task::new(id, description);
        let payload = TaskPayload { task_id: id, description: description.to_string() };
        (task, payload)
    }

    /// Receives the completed result from the Debugger and presents the final output.
    /// Reports honestly whether validation passed or not.
    pub fn receive_result(&self, mut task: Task, result: FinalPayload, validation_passed: bool) {
        task.status = TaskStatus::Complete;
        task.display_status();

        // Get the task description for file writing
        let task_description = self.task_descriptions
            .get(&result.task_id)
            .map(|s| s.as_str())
            .unwrap_or("Unknown task");

        // Write project to disk
        match file_writer::write_project(task_description, &result.code, result.task_id) {
            Ok(output) => {
                println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[32mTask #{} complete!\x1b[0m", result.task_id);
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m Summary: {}", result.summary);
                println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[1m✓ Project written to disk:\x1b[0m");
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m   {}", output.directory.display());
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m");
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[1mFiles created:\x1b[0m");
                for file in &output.files_created {
                    println!("\x1b[1;32m[COORDINATOR]\x1b[0m   • {}", file);
                }
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m");
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[1mTo run your project:\x1b[0m");
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m   cd {}", output.directory.display());
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m   cargo run");

                // Still show code preview
                println!("\n\x1b[1m{}\x1b[0m", "═".repeat(50));
                println!("\x1b[1m         CODE PREVIEW\x1b[0m");
                println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
                println!("{}", result.code);
                println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
            }
            Err(e) => {
                println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[31mError writing project to disk: {}\x1b[0m", e);
                println!("\x1b[1;32m[COORDINATOR]\x1b[0m Showing code output instead:\n");
                // Fallback to current behavior (print to stdout)
                println!("\n\x1b[1m{}\x1b[0m", "═".repeat(50));
                println!("\x1b[1m         FINAL OUTPUT CODE\x1b[0m");
                println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
                println!("{}", result.code);
                println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
            }
        }

        if !validation_passed {
            println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[33mNote: Validation did not fully pass — this is best-effort output.\x1b[0m");
        }
        println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m All agents finished. Pipeline complete.");
    }
}
