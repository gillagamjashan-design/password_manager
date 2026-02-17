# Template

Scaffold all files defined in the current project plan. Reads the oldest plan in `plans/`, extracts the file list, and creates each file with a starter template — enough code to show the structure but leaving full implementation for `/implement`.

## Variables

plan_path: (auto-detected — oldest `.md` file in `plans/`, by creation/modification date)

---

## Instructions

### Phase 1: Find and Read the Plan

1. List all `.md` files in `plans/` and select the oldest one (earliest modification date).
2. Read the entire plan file.
3. Locate the section `### New Files to Create` — this contains a markdown table with two columns: `File Path` and `Purpose`.
4. Extract every row from that table (skip the header row). Each row gives you a file to create.

---

### Phase 2: Create Each File with a Starter Template

For each file in the extracted list, create the file at the exact path listed in the table. Write starter template code appropriate for the file type. Use the rules below.

**Template rules by file type:**

**`Cargo.toml`**
Write a valid minimal Cargo.toml using details from the plan (project name, description, author if mentioned). Use `edition = "2021"`. Include a `[dependencies]` section with a comment if no dependencies are needed.

**`README.md`**
Write a minimal README with:
- A `# Project Title` heading (use the project name from the plan)
- A one-line description pulled from the plan's Overview section
- A `## Getting Started` section with placeholder instructions (`cargo build`, `cargo run`)
- A `## Project Structure` section listing the files being created

**`src/main.rs`**
Write:
```rust
// Entry point for <project name from plan>
// See README.md for build and run instructions

mod agents;
mod messages;
mod pipeline;
mod task;

fn main() {
    // TODO: wire up the pipeline and run sample tasks
    println!("Agent team starting...");
}
```

**`src/messages.rs`**
Write:
```rust
// Message types that agents use to communicate with each other
// TODO: define AgentMessage enum and payload structs

#[derive(Debug, Clone)]
pub struct TaskPayload {
    pub task_id: u32,
    pub description: String,
}
```

**`src/task.rs`**
Write:
```rust
// Tracks a task as it moves through the agent pipeline
// TODO: add TaskStatus enum and full Task implementation

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
}

impl Task {
    pub fn new(id: u32, description: &str) -> Self {
        Task { id, description: description.to_string() }
    }
}
```

**`src/pipeline.rs`**
Write:
```rust
// Wires all agents together in sequence
// TODO: implement the full Pipeline struct and run() method

pub struct Pipeline;

impl Pipeline {
    pub fn new() -> Self {
        Pipeline
    }

    pub fn run(&mut self, task_description: &str) {
        // TODO: pass task through all agents in order
        println!("Pipeline running task: {}", task_description);
    }
}
```

**`src/agents/mod.rs`**
Write:
```rust
// Agent module — each agent is in its own file
// TODO: uncomment modules as you implement each agent

pub mod coordinator;
pub mod planner;
pub mod coder;
pub mod reviewer;
pub mod debugger;
```

**Any `src/agents/<name>.rs` file** (coordinator, planner, coder, reviewer, debugger):
Use the agent name from the filename. Write:
```rust
// <AgentName> agent
// Purpose: <copy the Purpose column from the plan table>
// TODO: implement the full agent logic

pub struct <AgentName>Agent;

impl <AgentName>Agent {
    pub fn new() -> Self {
        <AgentName>Agent
    }

    pub fn process(&self) {
        // TODO: implement process() with correct input/output types
        println!("[<AGENTNAME>] Processing...");
    }
}
```
Replace `<AgentName>` with the capitalized agent name (e.g., `Coordinator`, `Planner`).
Replace `<AGENTNAME>` with the uppercase agent name (e.g., `COORDINATOR`, `PLANNER`).
Replace the purpose comment with the text from the Purpose column in the plan table.

**Any other `.rs` file not matched above:**
Write:
```rust
// <filename without extension>
// Purpose: <copy Purpose from plan table>
// TODO: implement this module
```

**Any other file type not listed:**
Create the file with a single comment line: `// TODO: implement <filename>`

---

### Phase 3: Validate

After creating all files:

1. List all files that were created and confirm they exist.
2. Confirm no files from the plan table were skipped.
3. If the project is a Rust project, note that `cargo check` can be run after `/implement` adds the full code — the templates alone may not compile because of `TODO` stubs.

---

## Report

After scaffolding, provide:

1. **Plan used:** filename of the plan that was read
2. **Files created:** list every file path that was created
3. **Next step:** remind the user to run `/implement plans/<plan-filename>.md` to fill in the full implementation
