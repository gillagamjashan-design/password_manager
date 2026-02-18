use crate::ai_client::call_gpt;
use crate::messages::{CodePayload, PlanPayload};

/// The Coder agent writes Rust code based on the plan and task description.
/// Powered by: GPT-4o (OpenAI) — Coding specialist.
pub struct CoderAgent;

impl CoderAgent {
    pub fn new() -> Self { CoderAgent }

    pub fn process_with_task(&self, plan: PlanPayload, task_description: &str) -> CodePayload {
        println!("\n\x1b[1;34m[CODER]\x1b[0m Received plan with {} steps.", plan.steps.len());
        println!("\x1b[1;34m[CODER]\x1b[0m \x1b[2m· Brain: GPT-4o (Coding)\x1b[0m");
        println!("\x1b[1;34m[CODER]\x1b[0m Writing Rust code for: \"{}\"", task_description);

        let code = self.generate_code(&plan.steps, task_description);

        println!("\x1b[1;34m[CODER]\x1b[0m Code written:");
        println!("\x1b[90m{}\x1b[0m", code);
        println!("\x1b[1;34m[CODER]\x1b[0m Handing off to Reviewer.");

        CodePayload {
            task_id: plan.task_id,
            code,
            language: "rust".to_string(),
        }
    }

    fn generate_code(&self, steps: &[String], task_description: &str) -> String {
        let steps_text = steps.join("\n");
        let system = "You are a Rust coding specialist. \
            Write clean, working Rust code. \
            Always include a main() function with example usage. \
            Add comments explaining what each function does. \
            Return ONLY the Rust code — no markdown, no backticks, no explanation text.";

        let user = format!(
            "Task: {task_description}\n\nImplementation steps:\n{steps_text}\n\nWrite the complete Rust code."
        );

        match call_gpt(system, &user) {
            Ok(code) => self.clean_code_response(&code),
            Err(e) => {
                println!("\x1b[1;34m[CODER]\x1b[0m \x1b[33mGPT unavailable: {e}\x1b[0m");
                println!("\x1b[1;34m[CODER]\x1b[0m \x1b[33mUsing fallback code.\x1b[0m");
                self.fallback_code(task_description)
            }
        }
    }

    /// Strips markdown code fences if the AI wrapped the response in them.
    fn clean_code_response(&self, code: &str) -> String {
        let trimmed = code.trim();
        // Remove ```rust ... ``` or ``` ... ``` wrappers if present
        if trimmed.starts_with("```") {
            let inner = trimmed
                .trim_start_matches("```rust")
                .trim_start_matches("```")
                .trim_end_matches("```");
            return inner.trim().to_string();
        }
        trimmed.to_string()
    }

    fn fallback_code(&self, task_description: &str) -> String {
        format!(
            "// Task: {task_description}\n// GPT-4o unavailable — set OPENAI_API_KEY to enable real code generation.\n\nfn main() {{\n    println!(\"Agent task: {task_description}\");\n}}"
        )
    }
}
