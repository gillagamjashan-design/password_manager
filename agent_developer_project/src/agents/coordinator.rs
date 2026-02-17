use crate::messages::{FinalPayload, TaskPayload};
use crate::task::{Task, TaskStatus};

/// The Coordinator agent manages the entire pipeline.
/// It assigns tasks to the team and receives the final result.
/// Think of this as the team lead or project manager.
pub struct CoordinatorAgent {
    task_counter: u32,
}

impl CoordinatorAgent {
    pub fn new() -> Self {
        CoordinatorAgent { task_counter: 0 }
    }

    /// Creates a new task and dispatches it to the pipeline.
    /// Returns a TaskPayload that the Planner will receive.
    pub fn assign_task(&mut self, description: &str) -> (Task, TaskPayload) {
        self.task_counter += 1;
        let id = self.task_counter;

        println!("\n[COORDINATOR] New task assigned.");
        println!("[COORDINATOR]   ID: #{}", id);
        println!("[COORDINATOR]   Task: \"{}\"", description);
        println!("[COORDINATOR] Dispatching to Planner...");

        let task = Task::new(id, description);
        let payload = TaskPayload {
            task_id: id,
            description: description.to_string(),
        };

        (task, payload)
    }

    /// Receives the completed result from the Debugger and presents the final output.
    pub fn receive_result(&self, mut task: Task, result: FinalPayload) {
        task.status = TaskStatus::Complete;
        task.display_status();

        println!("\n[COORDINATOR] Task #{} complete!", result.task_id);
        println!("[COORDINATOR] Summary: {}", result.summary);
        println!("\n========================================");
        println!("         FINAL OUTPUT CODE");
        println!("========================================");
        println!("{}", result.code);
        println!("========================================");
        println!("\n[COORDINATOR] All agents finished. Pipeline complete.");
    }
}
