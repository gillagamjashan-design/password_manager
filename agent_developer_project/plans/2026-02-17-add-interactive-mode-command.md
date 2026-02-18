# Plan: Add /interactive-mode Command

**Created:** 2026-02-17
**Status:** Draft
**Request:** Create a new /interactive-mode command that launches an agent which makes the agent team interactive — it waits for user input before running tasks, then loops until the user is done.

---

## Overview

### What This Plan Accomplishes

This plan adds a `/interactive-mode` slash command to the workspace and adds an `interactive` run mode to the Rust `agent-team` binary. When invoked from Claude Code, an agent reads the command, builds the project, runs it with an `--interactive` flag, and the binary enters a read-eval loop: it prints a prompt, waits for the user to type a task, runs the full agent pipeline on it, and repeats until the user types `exit` or `quit`.

### Why This Matters

Right now the agent team only runs fixed hardcoded tasks defined in `main.rs`. Interactive mode turns it into a real tool the user can drive themselves — type any coding task and instantly see all five agents (Coordinator, Planner, Coder, Reviewer, Debugger) work on it. This is the "supercoder assistant" experience the project was built toward.

---

## Current State

### Relevant Existing Structure

- `src/main.rs` — runs three hardcoded tasks; no user input handling
- `src/pipeline.rs` — `Pipeline::run(&str)` already accepts any task string
- `.claude/commands/` — all existing commands follow the same markdown format
- `plans/2026-02-17-rust-multi-agent-coding-system.md` — the oldest plan; needs this feature appended

### Gaps or Problems Being Addressed

- The binary cannot accept user-provided tasks at runtime; hardcoded tasks only
- There is no interactive loop; every run is fixed and exits immediately
- There is no `/interactive-mode` slash command to invoke this from Claude Code

---

## Proposed Changes

### Summary of Changes

- Add `--interactive` flag handling to `src/main.rs` using `std::env::args()`
- Add an `interactive_loop()` function in `src/main.rs` that reads stdin line-by-line and runs the pipeline on each input
- Add `.claude/commands/interactive-mode.md` — the slash command file
- Append a feature summary to `plans/2026-02-17-rust-multi-agent-coding-system.md`
- Update `CLAUDE.md` to document the new command

### New Files to Create

| File Path | Purpose |
| --- | --- |
| `.claude/commands/interactive-mode.md` | Slash command: builds the project and launches interactive agent-team session |

### Files to Modify

| File Path | Changes |
| --- | --- |
| `src/main.rs` | Add `--interactive` flag detection and `interactive_loop()` function |
| `plans/2026-02-17-rust-multi-agent-coding-system.md` | Append feature summary section |
| `CLAUDE.md` | Add `/interactive-mode` to the Commands section |

---

## Design Decisions

### Key Decisions Made

1. **`--interactive` CLI flag instead of a separate binary**: Keeps everything in one binary; `cargo run` stays the same; interactive mode is opt-in via flag.
2. **`std::env::args()` for flag parsing — no external crate**: Stays consistent with the project's no-external-dependencies philosophy (Cargo.toml has no deps).
3. **Slash command invokes an agent that builds + runs the binary**: Follows the same pattern as `/project-testing` — Claude Code builds the project, then runs it, then hands control to the interactive process.
4. **Loop until `exit` or `quit` (case-insensitive)**: Beginner-friendly exit mechanism; no Ctrl+C required.
5. **Empty input is skipped gracefully**: Prevents the pipeline from running on blank lines.
6. **Append to oldest plan after implementation**: Requested in the user's brief; keeps plan history consistent.

### Alternatives Considered

- **Separate `interactive` subcommand via a `commands` enum**: More idiomatic in large CLI apps but adds complexity for a beginner project; simple flag is sufficient.
- **Reading tasks from a file**: Not interactive; doesn't match the user's intent.
- **A separate binary target in Cargo.toml**: Unnecessary; one binary is cleaner.

### Open Questions (if any)

None — the approach is clear from existing patterns.

---

## Step-by-Step Tasks

### Step 1: Modify `src/main.rs` to support `--interactive` flag

Add argument parsing at the top of `main()`. If `--interactive` is present, call a new `interactive_loop()` function instead of running the hardcoded tasks. Add the `interactive_loop()` function after `main()`.

**Actions:**

- Read current `src/main.rs`
- Replace the body of `main()` so that after printing the banner it checks `std::env::args()` for `--interactive`
- If found, call `interactive_loop()`
- If not found, run the existing three hardcoded tasks (no change to existing behaviour)
- Add `interactive_loop()` function below `main()`

**Full new `src/main.rs` content:**

```rust
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
// Run with --interactive to enter interactive mode where you
// type your own tasks and the agent team works on them live.
// Type "exit" or "quit" to leave interactive mode.

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

    // Check if the user passed --interactive
    let args: Vec<String> = std::env::args().collect();
    let is_interactive = args.contains(&"--interactive".to_string());

    if is_interactive {
        interactive_loop();
    } else {
        // -------------------------------------------------------
        // Run Task 1: Prime number checker
        // -------------------------------------------------------
        let mut pipeline = Pipeline::new();

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
        println!("To run your own task, use: cargo run -- --interactive");
    }
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
```

**Files affected:**

- `src/main.rs`

---

### Step 2: Create `.claude/commands/interactive-mode.md`

Create the slash command file following the exact format of existing commands. When invoked, Claude will build the project with `cargo build` and then run it with `cargo run -- --interactive`.

**Full file content:**

```markdown
# Interactive Mode

Launch the agent team in interactive mode. The binary will start, print a prompt, and
wait for you to type a coding task. The full agent pipeline runs on whatever you type.
Type "exit" or "quit" to stop.

## Run

cargo build && cargo run -- --interactive
```

**Files affected:**

- `.claude/commands/interactive-mode.md`

---

### Step 3: Update `CLAUDE.md` — add `/interactive-mode` to Commands section

Add the new command entry in the Commands section of `CLAUDE.md`, after the `/add-installable-path` entry and before the closing `---` of the Commands section.

**New entry to add:**

```markdown
### /interactive-mode

**Purpose:** Launch the agent team in interactive mode — type any coding task and the agents work on it live.

When invoked, Claude will:

1. Run `cargo build` to ensure the binary is up to date
2. Run `cargo run -- --interactive` to start the interactive session
3. The agent team prints a prompt and waits for your input
4. Type any coding task (e.g., "write a function that reverses a string") and press Enter
5. All five agents work on your task and print their output
6. Type another task to continue, or type `exit` to stop

Use this when you want to give the agent team your own tasks instead of the built-in sample tasks.
```

**Files affected:**

- `CLAUDE.md`

---

### Step 4: Append feature summary to the oldest plan

The oldest plan is `plans/2026-02-17-rust-multi-agent-coding-system.md`. Append a new section at the end documenting this feature addition.

**Content to append:**

```markdown
## Feature Added: Interactive Mode (2026-02-17)

`src/main.rs` now supports a `--interactive` flag. When run as `cargo run -- --interactive`
(or `agent-team --interactive` if installed), the binary enters an interactive read-eval loop:
- Prints a prompt: `Your task >`
- Waits for the user to type any coding task
- Runs the full five-agent pipeline (Coordinator → Planner → Coder → Reviewer → Debugger) on the input
- Loops until the user types `exit` or `quit`

The `/interactive-mode` slash command in Claude Code builds the project and launches this mode automatically.
```

**Files affected:**

- `plans/2026-02-17-rust-multi-agent-coding-system.md`

---

### Step 5: Build and verify

Run `cargo build` to confirm the project compiles with no errors after the `src/main.rs` changes.

**Actions:**

- Run `cargo build`
- Confirm output contains `Compiling agent-team` and `Finished` with no errors
- If there are errors, fix them before continuing

**Files affected:**

- None (build step only)

---

### Step 6: Commit all changes

Commit the new command file, the updated `main.rs`, `CLAUDE.md`, and the oldest plan update.

**Actions:**

- `git add src/main.rs .claude/commands/interactive-mode.md CLAUDE.md plans/2026-02-17-rust-multi-agent-coding-system.md`
- `git commit -m "feat: add --interactive flag and /interactive-mode command"`

**Files affected:**

- All modified files listed above

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — lists all commands; must be updated
- `README.md` — describes how to run the project; may benefit from a note about `--interactive`, but not required for this plan
- `plans/2026-02-17-rust-multi-agent-coding-system.md` — feature log for the binary

### Updates Needed for Consistency

- `CLAUDE.md` Commands section must include `/interactive-mode`
- Oldest plan must include the feature summary (Step 4)

### Impact on Existing Workflows

- **No breaking changes**: `cargo run` without flags runs exactly as before (three hardcoded tasks)
- **`/project-testing`**: The behavioral expectations in the oldest plan cover the non-interactive run; interactive mode is separate and does not affect those tests
- **`agent-team` binary** (if installed via `/add-installable-path`): The installed binary automatically gets the new flag since it is rebuilt from `src/main.rs`

---

## Validation Checklist

- [ ] `cargo build` completes with no errors
- [ ] `cargo run` (no flags) still prints the three sample tasks and exits normally
- [ ] `cargo run -- --interactive` prints the interactive mode banner and the `Your task >` prompt
- [ ] Typing a task (e.g., "write a function that reverses a string") runs all five agents and prints output
- [ ] Typing `exit` exits cleanly with the goodbye message
- [ ] Typing `quit` exits cleanly with the goodbye message
- [ ] Empty input (just pressing Enter) is skipped without error
- [ ] `.claude/commands/interactive-mode.md` exists and follows the correct format
- [ ] `CLAUDE.md` contains the `/interactive-mode` entry in the Commands section
- [ ] Oldest plan has the feature summary appended
- [ ] All changes committed to git

---

## Success Criteria

The implementation is complete when:

1. `cargo run -- --interactive` starts an interactive session where the user can type any task and see all five agents work on it
2. The session loops indefinitely until `exit` or `quit` is typed
3. `/interactive-mode` is documented in CLAUDE.md and callable as a slash command in Claude Code
4. The oldest plan records this feature addition
5. All changes are committed to git

---

## Notes

- The `Pipeline::new()` is created once before the loop; this means the `task_counter` in `CoordinatorAgent` increments across tasks within a session, which is desirable (Task #1, #2, #3... as the user works)
- `io::stdout().flush()` is needed before `read_line` so the `Your task >` prompt appears before the user types (stdout is line-buffered by default on most terminals)
- Future enhancement: `pipeline.reset()` method to reset the task counter between sessions, but not needed now
