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
//   Validator   — confirms the output actually matches the task
//
// Type any coding task and press Enter. Type "exit" to quit.

mod agents;
mod file_writer;
mod messages;
mod pipeline;
mod task;

use pipeline::Pipeline;
use std::io::{self, BufRead, Write};

// ANSI color/style helpers
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";

fn main() {
    print_header();
    interactive_loop();
}

fn print_header() {
    println!();
    println!("  {BOLD}{CYAN}Agent Team{RESET} — Multi-Agent Coding Assistant");
    println!("  {DIM}Built in Rust · Six specialized agents{RESET}");
    println!();
    println!("  {DIM}Type a coding task and press Enter.{RESET}");
    println!("  {DIM}Type {BOLD}exit{RESET}{DIM} or {BOLD}quit{RESET}{DIM} to stop.{RESET}");
    println!();
    println!("  {DIM}─────────────────────────────────────────{RESET}");
    println!();
}

fn interactive_loop() {
    let mut pipeline = Pipeline::new();
    let stdin = io::stdin();

    loop {
        // Styled prompt — green arrow, like Claude CLI / Gemini CLI
        print!("  {BOLD}{GREEN}❯{RESET} ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => {
                println!("\n  {DIM}Goodbye!{RESET}");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                println!("  Error reading input: {e}");
                break;
            }
        }

        let task = input.trim();

        if task.is_empty() {
            continue;
        }

        if task.eq_ignore_ascii_case("exit") || task.eq_ignore_ascii_case("quit") {
            println!("\n  {DIM}Goodbye! Thanks for using Agent Team.{RESET}\n");
            break;
        }

        // Task start divider
        println!();
        println!("  {BOLD}{YELLOW}Working on your task...{RESET}");
        println!("  {DIM}─────────────────────────────────────────{RESET}");

        pipeline.run(task);

        // Task end divider
        println!();
        println!("  {DIM}─────────────────────────────────────────{RESET}");
        println!("  {BOLD}{GREEN}Done!{RESET} Ready for your next task.");
        println!();
    }
}
