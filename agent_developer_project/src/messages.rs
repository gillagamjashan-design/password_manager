/// Messages are how agents talk to each other.
/// Each variant represents a different stage of the pipeline.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AgentMessage {
    /// The coordinator sends a raw task to the planner
    TaskAssigned(TaskPayload),
    /// The planner sends a broken-down plan to the coder
    PlanReady(PlanPayload),
    /// The coder sends written code to the reviewer
    CodeReady(CodePayload),
    /// The reviewer sends feedback to the debugger
    ReviewComplete(ReviewPayload),
    /// The debugger sends the final fixed code back to the coordinator
    DebuggingComplete(FinalPayload),
}

/// A task given to the agent team to work on
#[derive(Debug, Clone)]
pub struct TaskPayload {
    pub task_id: u32,
    pub description: String,
}

/// The planner's output: a list of steps to follow
#[derive(Debug, Clone)]
pub struct PlanPayload {
    pub task_id: u32,
    pub steps: Vec<String>,
}

/// The coder's output: the written code
#[derive(Debug, Clone)]
pub struct CodePayload {
    pub task_id: u32,
    pub code: String,
    #[allow(dead_code)]
    pub language: String,
}

/// The reviewer's output: a list of issues found (empty = all good)
#[derive(Debug, Clone)]
pub struct ReviewPayload {
    pub task_id: u32,
    pub code: String,
    pub issues: Vec<String>,
    pub approved: bool,
}

/// The final output after debugging
#[derive(Debug, Clone)]
pub struct FinalPayload {
    pub task_id: u32,
    pub code: String,
    pub summary: String,
}

/// The validator's output: did the final code match the user's task?
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ValidationPayload {
    pub task_id: u32,
    pub passed: bool,
    pub reason: String,
}
