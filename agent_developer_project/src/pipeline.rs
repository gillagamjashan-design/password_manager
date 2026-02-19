use crate::agents::coder::CoderAgent;
use crate::agents::coordinator::CoordinatorAgent;
use crate::agents::debugger::DebuggerAgent;
use crate::agents::planner::PlannerAgent;
use crate::agents::reviewer::ReviewerAgent;
use crate::agents::validator::ValidatorAgent;
use crate::task::TaskStatus;
use crate::thinking::{ThinkingTimer, ProcessingStage};

/// The Pipeline connects all agents in sequence.
/// Running the pipeline on a task description takes it through all stages:
/// Coordinator → Planner → Coder → Reviewer → Debugger → Validator → Coordinator
/// If Validator fails, Coder and Debugger retry up to 3 times.
pub struct Pipeline {
    coordinator: CoordinatorAgent,
    planner: PlannerAgent,
    coder: CoderAgent,
    reviewer: ReviewerAgent,
    debugger: DebuggerAgent,
    validator: ValidatorAgent,
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
            validator: ValidatorAgent::new(),
        }
    }

    /// Runs a task description through the full agent pipeline.
    /// Each agent processes the output of the previous one.
    /// If validation fails, the Coder/Debugger stages retry up to 5 times.
    pub fn run(&mut self, task_description: &str) {
        const MAX_RETRIES: u32 = 5;

        println!("\n  \x1b[1;35m[PIPELINE]\x1b[0m Starting work on your task...");

        // Stage 1: Coordinator assigns the task
        let (mut task, task_payload) = self.coordinator.assign_task(task_description);
        task.status = TaskStatus::Planning;
        // task.display_status(); // Removed - replaced by thinking timers

        // Stage 2: Planner breaks it into steps (runs once; the plan doesn't change on retry)
        let plan = self.planner.process(task_payload.clone());

        // Transition delay after planning
        ThinkingTimer::new(ProcessingStage::Planning, 5).start();

        task.status = TaskStatus::Coding;
        // task.display_status(); // Removed - replaced by thinking timers

        let mut attempt = 0u32;
        let mut enriched_description = task_description.to_string();

        loop {
            attempt += 1;

            if attempt > 1 {
                println!(
                    "\n\x1b[1;35m[PIPELINE]\x1b[0m Retry attempt {}/{} - Taking extra time to ensure quality...",
                    attempt - 1,
                    MAX_RETRIES
                );
            }

            // Stage 3: Coder writes the code (uses enriched description on retry)
            let mut retry_plan = plan.clone();
            if attempt > 1 {
                // Append validator feedback to steps so coder has more signal
                retry_plan.steps.push(format!(
                    "IMPORTANT: Previous attempt did not address '{}'. Make sure the function name and logic relate directly to this task.",
                    enriched_description
                ));
            }
            let code = self.coder.process_with_task(retry_plan, &enriched_description);

            // Transition delay after coding
            ThinkingTimer::new(ProcessingStage::StaticReview, 5).start();

            task.status = TaskStatus::Reviewing;
            // task.display_status(); // Removed - replaced by thinking timers

            // Stage 4: Reviewer checks for issues
            let review = self.reviewer.process(code);

            // Transition delay after reviewing
            ThinkingTimer::new(ProcessingStage::Debugging, 5).start();

            task.status = TaskStatus::Debugging;
            // task.display_status(); // Removed - replaced by thinking timers

            // Stage 5: Debugger fixes any issues
            let final_result = self.debugger.process(review);

            // Stage 6: Validator checks if output matches the task
            let validation = self.validator.process(&final_result, &enriched_description);

            if validation.passed || attempt >= MAX_RETRIES {
                if !validation.passed {
                    println!(
                        "\n\x1b[1;35m[PIPELINE]\x1b[0m \x1b[33mMax retries reached ({}). Reporting best available output.\x1b[0m",
                        MAX_RETRIES
                    );
                }
                // Stage 7: Coordinator receives and presents the result
                self.coordinator.receive_result(task, final_result, validation.passed);
                break;
            }

            // Enrich description with retry context for next attempt
            enriched_description = format!("{} (focus on: {})", task_description, enriched_description);
        }
    }
}
