// ============================================================
// agent-team: A multi-agent coding assistant written in Rust
// ============================================================
//
// HOW IT WORKS:
// This program simulates a team of AI agents that work together
// like software engineers to complete coding tasks.
//
// The agents and their roles:
//   Coordinator — assigns tasks and receives final output
//   Planner     — breaks the task into implementation steps
//   Coder       — writes Rust code based on the plan
//   Reviewer    — checks the code for quality issues
//   Debugger    — fixes any issues the reviewer found
//
// Each agent prints what it's doing, so you can follow along!

mod agents;
mod messages;
mod pipeline;
mod task;

use pipeline::Pipeline;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║      AGENT TEAM — Coding Assistant   ║");
    println!("║      Built in Rust                   ║");
    println!("╚══════════════════════════════════════╝");

    // Create the pipeline (this sets up all 5 agents)
    let mut pipeline = Pipeline::new();

    // -------------------------------------------------------
    // Run Task 1: Prime number checker
    // -------------------------------------------------------
    println!("\n\n★★★ STARTING TASK 1 ★★★");
    pipeline.run("write a function that checks if a number is prime");

    println!("\n\n{}", "=".repeat(60));

    // -------------------------------------------------------
    // Run Task 2: Sorting algorithm
    // -------------------------------------------------------
    println!("\n★★★ STARTING TASK 2 ★★★");
    pipeline.run("write a function that sorts an array of numbers");

    println!("\n\n{}", "=".repeat(60));

    // -------------------------------------------------------
    // Run Task 3: Fibonacci sequence
    // -------------------------------------------------------
    println!("\n★★★ STARTING TASK 3 ★★★");
    pipeline.run("write a function that returns the fibonacci sequence");

    println!("\n\n{}", "=".repeat(60));
    println!("\nAll tasks complete. The agent team has finished its work.");
    println!("To run your own task, modify the pipeline.run() calls in src/main.rs");
}
