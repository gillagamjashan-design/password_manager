# Agent Team — Multi-Agent Coding Assistant in Rust

A multi-agent coding assistant where specialized agents work together like a software engineering team to complete coding tasks.

---

## What It Does

This program runs a team of 5 AI agents that pass work to each other in a pipeline:

```
Coordinator → Planner → Coder → Reviewer → Debugger → Coordinator
```

You give the team a coding task (like "write a function that checks if a number is prime"), and the agents collaborate to plan, write, review, and fix the code — then present the final result.

Each agent prints what it's doing as it works, so you can follow the whole process.

---

## The Agents

| Agent | Role |
|-------|------|
| **Coordinator** | Assigns the task and presents the final output |
| **Planner** | Breaks the task into clear implementation steps |
| **Coder** | Writes Rust code based on the plan |
| **Reviewer** | Checks the code for quality issues |
| **Debugger** | Fixes any issues the reviewer found |

---

## Prerequisites

You need Rust installed. If you don't have it yet:

1. Go to [https://rustup.rs](https://rustup.rs)
2. Follow the instructions for your operating system
3. Verify installation by running: `rustc --version`

---

## How to Build and Run

**Step 1: Clone or download this project**

**Step 2: Open a terminal in the project folder**

**Step 3: Build the project**
```bash
cargo build
```

**Step 4: Run the program**
```bash
cargo run
```

That's it. The agent team will run 3 sample tasks and print all output to your terminal.

---

## How to Add Your Own Tasks

Open `src/main.rs` and add a new `pipeline.run()` call:

```rust
pipeline.run("write a function that checks if a string is a palindrome");
```

The agents will automatically handle it.

---

## Project Structure

```
agent-team/
├── Cargo.toml          ← Project config (like package.json in JavaScript)
├── README.md           ← This file
└── src/
    ├── main.rs         ← Entry point — runs the agent pipeline
    ├── messages.rs     ← Message types agents use to communicate
    ├── task.rs         ← Task struct and status tracking
    ├── pipeline.rs     ← Wires all agents together in sequence
    └── agents/
        ├── mod.rs          ← Module declarations
        ├── coordinator.rs  ← Coordinator agent
        ├── planner.rs      ← Planner agent
        ├── coder.rs        ← Coder agent
        ├── reviewer.rs     ← Reviewer agent
        └── debugger.rs     ← Debugger agent
```

---

## Understanding the Code

The agents communicate by passing data structs to each other. Here's the flow:

1. `Coordinator` creates a `TaskPayload` → sends to `Planner`
2. `Planner` returns a `PlanPayload` (list of steps) → sends to `Coder`
3. `Coder` returns a `CodePayload` (the written code) → sends to `Reviewer`
4. `Reviewer` returns a `ReviewPayload` (code + any issues found) → sends to `Debugger`
5. `Debugger` returns a `FinalPayload` (fixed code + summary) → back to `Coordinator`
6. `Coordinator` prints the final result

All message types are defined in `src/messages.rs`.

---

## Built With

- **Rust** — systems programming language known for safety and performance
- **Standard library only** — no external dependencies needed

---

## License

MIT
