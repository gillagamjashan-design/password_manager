# Plan: Add /template Command

**Created:** 2026-02-17
**Status:** Implemented
**Request:** Create a `/template` command that reads the project plan in `plans/` and generates all the files defined in that plan, each with starter template code.

---

## Overview

### What This Plan Accomplishes

This plan creates a new slash command `/template` for Claude Code. When run, Claude reads the oldest plan file in `plans/`, identifies every file listed in the plan's "New Files to Create" section, creates each file at its specified path, and writes a starter template (a small amount of boilerplate code) into each one. The result is a scaffolded project ready for full implementation.

### Why This Matters

Right now there is a detailed Rust project plan (`plans/2026-02-17-rust-multi-agent-coding-system.md`) but no actual source files exist yet. Running `/template` gives a fast, reliable way to scaffold the entire file structure in one shot — so the project has a working skeleton before `/implement` fills in the full code. This matches the goal of building a well-structured, beginner-friendly project and supports the workflow of plan → scaffold → implement.

---

## Current State

### Relevant Existing Structure

```
.claude/
└── commands/
    ├── prime.md        ← /prime command — session initialization
    ├── create-plan.md  ← /create-plan command — generates plan docs
    └── implement.md    ← /implement command — executes plan steps
plans/
└── 2026-02-17-rust-multi-agent-coding-system.md  ← the plan to scaffold from
```

Commands are markdown files in `.claude/commands/`. Each file's name becomes the slash command name (e.g., `template.md` → `/template`). Commands contain instructions that Claude follows when the command is invoked.

There is currently no `/template` command.

### Gaps or Problems Being Addressed

- No command exists to scaffold project files from a plan
- Going from a plan to a working file structure requires either running `/implement` (which writes full code) or manually creating files — there is no middle-ground "stub files" step
- A `/template` command fills this gap: creates files with minimal starter code so the project structure is visible and navigable before full implementation

---

## Proposed Changes

### Summary of Changes

- Create `.claude/commands/template.md` — the new `/template` command definition
- Update `CLAUDE.md` to list `/template` in the Commands section

### New Files to Create

| File Path | Purpose |
| --- | --- |
| `.claude/commands/template.md` | Defines the `/template` slash command and its instructions |

### Files to Modify

| File Path | Changes |
| --- | --- |
| `CLAUDE.md` | Add `/template` to the Commands table and Commands section |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **Command reads the oldest plan file automatically**: Rather than requiring the user to pass a plan path argument, the command finds the oldest `.md` file in `plans/`. This keeps usage simple — just `/template` with no arguments. The oldest plan is the original project plan — the one that defines the files to scaffold.

2. **Templates are language-aware stubs, not empty files**: Each file gets a minimal starter — for Rust files this means the correct `mod` declaration or a `struct` stub and a `// TODO` comment, not just a blank file. This gives the project a compilable skeleton that communicates intent.

3. **Command follows the same instruction style as existing commands**: The template command is written in the same imperative-instruction style as `prime.md`, `create-plan.md`, and `implement.md` — a Variables section, an Instructions section with numbered phases, and a Report section.

4. **Command targets the "New Files to Create" table in the plan**: The plan format (defined in `create-plan.md`) always has a section called `### New Files to Create` with a markdown table. The command instructs Claude to parse that table and use it as the file list.

5. **Starter templates are defined per file type in the command**: The command specifies what starter code to write for each type of file (`.rs` modules, `.rs` agent structs, `Cargo.toml`, `README.md`, etc.) so Claude doesn't have to guess — it follows a clear mapping.

### Alternatives Considered

- **Require a plan path argument** (like `/implement`): Rejected because the workspace has one active project at a time and always one current plan. No-argument is simpler.
- **Write fully empty files**: Rejected — empty files don't communicate structure or compile. Stubs with a clear shape are more useful as a starting point.
- **Have `/implement` handle scaffolding**: Rejected — `/implement` writes complete code and is designed to run the full plan. A separate `/template` command gives a lighter-weight first step that doesn't write all the code.

### Open Questions (if any)

None — the plan file format and workspace patterns are well understood. Implementation can proceed.

---

## Step-by-Step Tasks

### Step 1: Create `.claude/commands/template.md`

Create the command file that defines `/template`. The file instructs Claude to:
1. Find the most recently modified plan file in `plans/`
2. Read it and extract the "New Files to Create" table
3. For each file in the table, create the file at the listed path with starter template code
4. Report what was created

**Actions:**

- Write `.claude/commands/template.md` with the full content shown below

**Files affected:**

- `.claude/commands/template.md`

**Full content for `.claude/commands/template.md`:**

```markdown
# Template

Scaffold all files defined in the current project plan. Reads the newest plan in `plans/`, extracts the file list, and creates each file with a starter template — enough code to show the structure but leaving full implementation for `/implement`.

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
```

---

### Step 2: Update `CLAUDE.md` to Document `/template`

Add `/template` to the Commands section in `CLAUDE.md` so future sessions know this command exists.

**Actions:**

- Read `CLAUDE.md`
- Find the Commands section — it currently lists `/prime`, `/create-plan`, and `/implement`
- Add a new subsection for `/template` after `/implement`, following the same format as the other command entries

**Files affected:**

- `CLAUDE.md`

**Content to add** (insert after the `/implement` subsection, before `---` and Notes):

```markdown
### /template

**Purpose:** Scaffold all files defined in the current project plan with starter template code.

Run this after `/create-plan` and before `/implement` to create the file structure. Claude will:

1. Find the oldest plan in `plans/`
2. Read the "New Files to Create" table
3. Create each file at its listed path with minimal starter code (stubs, not full implementation)
4. Report what was created

This gives you a navigable project skeleton before running `/implement` to fill in the full code.
```

Also add `/template` to the Commands quick-reference table (if one exists in CLAUDE.md) or to the Session Workflow list as an optional step between `/create-plan` and `/implement`.

**Session Workflow update** — the list currently reads:
```
1. Start: Run /prime to load context
2. Work: Use commands or direct Claude with tasks
3. Plan changes: Use /create-plan before significant additions
4. Execute: Use /implement to execute plans
5. Maintain: Claude updates CLAUDE.md and context/ as the workspace evolves
```

Update to:
```
1. Start: Run /prime to load context
2. Work: Use commands or direct Claude with tasks
3. Plan changes: Use /create-plan before significant additions
4. Scaffold (optional): Use /template to create starter files from the plan
5. Execute: Use /implement to execute plans
6. Maintain: Claude updates CLAUDE.md and context/ as the workspace evolves
```

---

### Step 3: Commit the Changes

Commit the two changed files to git.

**Actions:**

- Stage `.claude/commands/template.md` and `CLAUDE.md`
- Commit with message: `feat: add /template command for scaffolding project files from plan`

**Files affected:**

- `.claude/commands/template.md`
- `CLAUDE.md`

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — lists all commands; must be updated to include `/template`
- `plans/2026-02-17-rust-multi-agent-coding-system.md` — the plan `/template` will read when first run

### Updates Needed for Consistency

- `CLAUDE.md` Commands section and Session Workflow section both need the new command added

### Impact on Existing Workflows

- `/template` slots between `/create-plan` and `/implement` in the workflow — it is optional but gives a useful scaffolding step
- `/implement` is unchanged — it still writes full code. `/template` just creates the skeleton first so the file structure is visible
- `/prime` will load `CLAUDE.md` and surface the new command to future sessions automatically

---

## Validation Checklist

- [ ] `.claude/commands/template.md` exists and contains all three phases (Find Plan, Create Files, Validate)
- [ ] Template rules cover all file types present in the Rust project plan (Cargo.toml, main.rs, messages.rs, task.rs, pipeline.rs, agents/mod.rs, all agent files, README.md)
- [ ] `CLAUDE.md` Commands section includes a `/template` entry
- [ ] `CLAUDE.md` Session Workflow is updated to include the scaffold step
- [ ] Both files are committed to git

---

## Success Criteria

The implementation is complete when:

1. `/template` appears as a documented command in `CLAUDE.md`
2. Running `/template` in a future Claude Code session causes Claude to read the plan, create all 12 files from the Rust plan, and write a valid starter template in each
3. The command file follows the same structure and style as `prime.md`, `create-plan.md`, and `implement.md`

---

## Notes

- The `/template` command is intentionally dumb about compilation — the stubs it creates may not compile on their own (e.g., `agents/mod.rs` declares modules that aren't fully implemented yet). That's fine: `/implement` is what makes it compile. `/template` just makes the file tree visible.
- In the future, `/template` could accept an optional plan path argument (like `/implement`) to handle workspaces with multiple active plans.
- The command could also be extended to detect the language from the plan and adjust templates accordingly (Python, JavaScript, etc.).

---

## Implementation Notes

**Implemented:** 2026-02-17

### Summary

Created `.claude/commands/template.md` with the full `/template` command definition (three phases: find plan, create files, validate). Updated `CLAUDE.md` to document the command in the Commands section, updated the Session Workflow to include the scaffold step, and updated the Workspace Structure tree. Both files committed to git.

### Deviations from Plan

- Step 1 description text in the plan had a stale reference to "most recently modified" — the actual command content was correct (oldest). Fixed in command file; plan description left as-is since it is documentation only.
- `.claude/` is excluded by a parent-level `.gitignore`. Used `git add -f` to force-add `template.md`, which is the correct approach for intentional workspace tooling files. No content was changed.

### Issues Encountered

- Parent repo `.gitignore` at `/workspace/jashan/.gitignore` blocks `.claude/` directory. Resolved with `git add -f`.
