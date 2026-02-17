use crate::agents::coder::CoderAgent;
use crate::agents::coordinator::CoordinatorAgent;
use crate::agents::debugger::DebuggerAgent;
use crate::agents::planner::PlannerAgent;
use crate::agents::reviewer::ReviewerAgent;
use crate::task::TaskStatus;

/// The Pipeline connects all agents in sequence.
/// Running the pipeline on a task description takes it through all stages:
/// Coordinator → Planner → Coder → Reviewer → Debugger → Coordinator
pub struct Pipeline {
    coordinator: CoordinatorAgent,
    planner: PlannerAgent,
    coder: CoderAgent,
    reviewer: ReviewerAgent,
    debugger: DebuggerAgent,
}

impl Pipeline {
    /// Creates a new pipeline with all agents initialized.
    pub fn new() -> Self {
        Pipeline {
            coordinator: CoordinatorAgent::new(),
            planner: PlannerAgent::new(),
            coder: CoderAgent::new(),
            reviewer: ReviewerAgent::new(),
            debugger: DebuggerAgent::new(),
        }
    }

    /// Runs a task description through the full agent pipeline.
    /// Each agent processes the output of the previous one.
    pub fn run(&mut self, task_description: &str) {
        // Stage 1: Coordinator assigns the task
        let (mut task, task_payload) = self.coordinator.assign_task(task_description);
        task.status = TaskStatus::Planning;
        task.display_status();

        // Stage 2: Planner breaks it into steps
        let plan = self.planner.process(task_payload);
        task.status = TaskStatus::Coding;
        task.display_status();

        // Stage 3: Coder writes the code
        let code = self.coder.process(plan);
        task.status = TaskStatus::Reviewing;
        task.display_status();

        // Stage 4: Reviewer checks for issues
        let review = self.reviewer.process(code);
        task.status = TaskStatus::Debugging;
        task.display_status();

        // Stage 5: Debugger fixes any issues
        let final_result = self.debugger.process(review);

        // Stage 6: Coordinator receives and presents the result
        self.coordinator.receive_result(task, final_result);
    }
}
