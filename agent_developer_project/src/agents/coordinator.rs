use crate::messages::{FinalPayload, TaskPayload};
use crate::task::{Task, TaskStatus};

/// The Coordinator agent manages the entire pipeline.
/// It assigns tasks to the team and receives the final result.
/// Think of this as the team lead or project manager.
pub struct CoordinatorAgent {
    task_counter: u32,
}

impl CoordinatorAgent {
    pub fn new() -> Self { CoordinatorAgent { task_counter: 0 } }

    /// Creates a new task and dispatches it to the pipeline.
    pub fn assign_task(&mut self, description: &str) -> (Task, TaskPayload) {
        self.task_counter += 1;
        let id = self.task_counter;

        println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m New task assigned.");
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[2m· Brain: Claude (Architecture)\x1b[0m");
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

        if validation_passed {
            println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[32mTask #{} complete!\x1b[0m", result.task_id);
        } else {
            println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[33mTask #{} — best effort output (validation did not fully pass)\x1b[0m", result.task_id);
        }
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m Summary: {}", result.summary);
        println!("\n\x1b[1m{}\x1b[0m", "═".repeat(50));
        println!("\x1b[1m         FINAL OUTPUT CODE\x1b[0m");
        println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
        println!("{}", result.code);
        println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
        println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m All agents finished. Pipeline complete.");
    }
}
