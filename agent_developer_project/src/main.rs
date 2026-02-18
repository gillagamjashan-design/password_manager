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
// The program starts in interactive mode by default — type any coding
// task and the agent team will work on it live.
// Type "exit" or "quit" to stop.

mod agents;
mod messages;
mod pipeline;
mod task;

use pipeline::Pipeline;
use std::io::{self, BufRead, Write};

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║      AGENT TEAM — Coding Assistant   ║");
    println!("║      Built in Rust                   ║");
    println!("╚══════════════════════════════════════╝");

    interactive_loop();
}

/// Interactive mode: waits for the user to type a task, runs the full
/// agent pipeline on it, then asks for the next task.
/// Type "exit" or "quit" to stop.
fn interactive_loop() {
    let mut pipeline = Pipeline::new();

    println!("\n╔══════════════════════════════════════╗");
    println!("║        INTERACTIVE MODE ACTIVE       ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  Type a coding task and press Enter  ║");
    println!("║  The agent team will work on it.     ║");
    println!("║  Type \"exit\" or \"quit\" to stop.      ║");
    println!("╚══════════════════════════════════════╝\n");

    let stdin = io::stdin();

    loop {
        // Print prompt
        print!("Your task > ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Read a line from the user
        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D) — exit gracefully
                println!("\n[AGENT TEAM] Input stream closed. Goodbye!");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                println!("[AGENT TEAM] Error reading input: {}", e);
                break;
            }
        }

        let task = input.trim();

        // Skip empty input
        if task.is_empty() {
            continue;
        }

        // Exit commands
        if task.eq_ignore_ascii_case("exit") || task.eq_ignore_ascii_case("quit") {
            println!("\n[AGENT TEAM] Goodbye! Thanks for using Agent Team.");
            break;
        }

        // Run the full pipeline on the user's task
        println!("\n{}", "=".repeat(60));
        println!("[AGENT TEAM] Starting work on your task...");
        println!("{}", "=".repeat(60));

        pipeline.run(task);

        println!("\n{}", "=".repeat(60));
        println!("[AGENT TEAM] Done! Ready for your next task.");
        println!("{}\n", "=".repeat(60));
    }
}
