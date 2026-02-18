/// Tracks a task as it moves through the agent pipeline
#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}

/// The lifecycle stages a task passes through
#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    Planning,
    Coding,
    Reviewing,
    Debugging,
    Complete,
}

impl Task {
    /// Create a new task with Pending status
    pub fn new(id: u32, description: &str) -> Self {
        Task {
            id,
            description: description.to_string(),
            status: TaskStatus::Pending,
        }
    }

    /// Print the current task status to the terminal
    pub fn display_status(&self) {
        println!(
            "\n  \x1b[2m[Task #{}] {:?} — {}\x1b[0m",
            self.id, self.status, self.description
        );
    }
}
