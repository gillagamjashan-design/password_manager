# Plan: Add /agent-fix Command — Make Agent Team Write Real Project Files

**Created:** 2026-02-18
**Status:** Draft
**Request:** Create /agent-fix command that spawns Claude Code agents to fix the agent-team binary so it actually creates project files on disk (Cargo.toml, src/*.rs, etc.) instead of just printing code to stdout

---

## Overview

### What This Plan Accomplishes

This plan creates a new `/agent-fix` slash command for Claude Code. When run, it spawns multiple Claude Code agents who collaboratively diagnose and fix the agent-team binary. The core issue: currently the agent-team prints generated code to the terminal and reports "done" instantly without creating any actual project files. After this fix, when you run `agent-team` and give it a task like "write a CLI calculator in Rust", it should create a real directory structure with Cargo.toml, src/main.rs, and working code that you can immediately `cd` into and run with `cargo run`.

### Why This Matters

The agent-team project is meant to be a practical coding assistant tool. Right now it generates code but doesn't write it to disk, making it essentially a "view-only" demo. To be genuinely useful for real-world development, it needs to create actual runnable projects in the filesystem. This command fixes that by having Claude Code agents architect and implement the file-writing functionality, then verify it works end-to-end.

---

## Current State

### Relevant Existing Structure

```
agent_developer_project/
├── .claude/commands/       # Existing slash commands
│   ├── fix.md             # Prior /fix command (UI improvements)
│   ├── refactor.md        # Prior /refactor command (removed AI APIs)
│   ├── brain-chips.md     # Prior /brain-chips command (added AI APIs)
│   └── ...
├── src/
│   ├── main.rs            # Entry point — runs interactive loop
│   ├── pipeline.rs        # Runs agents in sequence
│   ├── agents/
│   │   ├── coordinator.rs # Assigns tasks, displays output
│   │   ├── planner.rs     # Breaks tasks into steps
│   │   ├── coder.rs       # Generates Rust code
│   │   ├── reviewer.rs    # Checks code quality
│   │   ├── debugger.rs    # Fixes issues
│   │   └── validator.rs   # Confirms output matches task
│   ├── messages.rs        # Message payloads between agents
│   └── task.rs            # Task lifecycle tracking
├── Cargo.toml             # Rust project manifest
└── README.md              # Beginner-friendly docs
```

The agent-team currently:
- Runs interactively in a terminal loop
- Takes user tasks like "write a function that sorts numbers"
- Passes the task through 6 agents (Coordinator → Planner → Coder → Reviewer → Debugger → Validator)
- Prints the generated code to stdout in the Coordinator's final output
- Says "Done!" and waits for the next task

### Gaps or Problems Being Addressed

1. **No file writing** — All output goes to stdout. Nothing is written to disk.
2. **No project structure creation** — The agent-team doesn't create directories, Cargo.toml, src/ folders, or .rs files.
3. **Not usable for real projects** — You can't take the output and immediately run it. You have to manually copy/paste the code into files.
4. **Missing project context awareness** — The agents don't understand concepts like "initialize a Rust binary project" or "create a multi-file library structure".

After this fix:
- Agent-team will create a timestamped project directory (e.g., `agent-team-output-2026-02-18-14-23-05/`)
- It will write Cargo.toml, src/main.rs, and any other needed files to that directory
- It will print the path to the new project so you can immediately `cd` into it and run `cargo run`

---

## Proposed Changes

### Summary of Changes

- Create a new slash command file: `.claude/commands/agent-fix.md`
- Update `CLAUDE.md` Commands section to document the new `/agent-fix` command
- The command will spawn multiple Claude Code agents who:
  1. Read the current agent-team source code to understand its architecture
  2. Design a file-writing system (FileWriterAgent or output module)
  3. Implement the file-writing functionality in the codebase
  4. Update the Coordinator and Pipeline to call the file writer
  5. Test the changes by running `agent-team` with a sample task and verifying files are created
  6. Commit all changes to git

### New Files to Create

| File Path | Purpose |
| --------- | ------- |
| `.claude/commands/agent-fix.md` | Slash command that spawns agents to fix file-writing functionality |

### Files to Modify

| File Path | Changes |
| --------- | ------- |
| `CLAUDE.md` | Add `/agent-fix` to Commands section with description |

### Files to Delete (if any)

None. The fix will add new functionality to the existing agent-team source code, but Claude Code agents will determine exactly which source files need modification during execution.

---

## Design Decisions

### Key Decisions Made

1. **Use Claude Code agents, not modify the agent-team agents directly**: The `/agent-fix` command spawns external Claude Code agents who read, plan, and modify the Rust source code. This keeps the implementation aligned with the existing command patterns (`/refactor`, `/brain-chips`).

2. **File writer as a new module or agent**: The agents will decide whether to add a dedicated `FileWriterAgent` or create a `src/file_writer.rs` utility module. Both are valid; agents will choose based on architectural analysis.

3. **Output directory structure**: Each task run will create a timestamped directory (e.g., `agent-team-output-YYYY-MM-DD-HH-MM-SS/`) in the current working directory. This avoids conflicts when running multiple tasks.

4. **Always include Cargo.toml + src/main.rs at minimum**: Even if the task is simple ("function that adds two numbers"), the output will be a complete runnable Rust project.

5. **Coordinator prints the project path**: After file writing completes, the Coordinator's final output includes: "Project created at: ./agent-team-output-2026-02-18-14-23-05/" so the user knows exactly where to go.

6. **Agents make architectural decisions**: The Claude Code agents spawned by this command have full freedom to read the codebase, propose the best design, and implement it. The command defines goals, not implementation details.

7. **Sequential agent execution with checkpoints**: Agents run in sequence with validation checkpoints:
   - Agent 1 (Architect) reads and plans
   - Agent 2 (Implementer) writes the file-writing code
   - Agent 3 (Integrator) updates Coordinator and Pipeline to use the file writer
   - Agent 4 (Validator) tests end-to-end and commits

8. **All agents use Sonnet 4.5 (default)**: No need for Opus since this is a well-defined implementation task with clear success criteria.

### Alternatives Considered

- **Modify agents manually in this session**: Rejected — using the `/agent-fix` command pattern (spawning agents) keeps the workspace organized and provides a reusable pattern for future fixes.

- **Output to a single fixed directory like `output/`**: Rejected — using timestamped directories avoids overwriting previous runs and makes it easy to compare outputs.

- **Let the user specify the output directory**: Considered but deferred — for now, the agent-team will choose the directory name automatically. This can be added as a future enhancement.

- **Write files during Coder stage instead of after Coordinator**: Rejected — cleaner to keep code generation separate from file I/O. The Coordinator stage is the final "delivery" step, making it the logical place to write files.

### Open Questions (if any)

None — the command is fully specified and agents have clear goals.

---

## Step-by-Step Tasks

Execute these tasks in order during implementation.

### Step 1: Create the /agent-fix Command File

Write `.claude/commands/agent-fix.md` with the full command specification.

**Actions:**

- Create `.claude/commands/agent-fix.md`
- Define the command's purpose, what it fixes, and how it spawns agents
- Specify 4 agents with clear roles and prompts
- Include validation steps and success criteria

**Files affected:**

- `.claude/commands/agent-fix.md` (new)

**Content for `.claude/commands/agent-fix.md`:**

```markdown
# Agent Fix

Fix the agent-team binary so it creates real project files on disk instead of just printing code to stdout.

## What This Fixes

**Problem:** Currently, when you run `agent-team` and give it a task (e.g., "write a CLI calculator"), it generates code and prints it to the terminal, then says "Done!" without writing any files. You can't immediately use the output — you'd have to manually copy the code into files.

**After this fix:** The agent-team will create a timestamped project directory (e.g., `agent-team-output-2026-02-18-14-23-05/`) with a complete Rust project inside: `Cargo.toml`, `src/main.rs`, and any other needed files. You can immediately `cd` into the directory and run `cargo run`.

---

## Run

Spawn 4 agents in sequence. Each agent uses the Task tool with `subagent_type="general-purpose"` and default model (Sonnet 4.5).

**IMPORTANT:** Agents run sequentially. Agent 2 starts after Agent 1 finishes. Agent 3 starts after Agent 2 finishes. Agent 4 starts after Agent 3 finishes.

---

### Agent 1 (Sonnet 4.5) — Architect

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"`.

**Prompt to pass:**

```
You are the Architect agent fixing the agent-team project to write real files to disk.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Read the agent-team source code and design a file-writing system.

**Steps:**

1. Read these files to understand the current architecture:
   - src/main.rs
   - src/pipeline.rs
   - src/agents/coordinator.rs
   - src/agents/coder.rs
   - src/messages.rs
   - Cargo.toml

2. Identify where the final code output happens (hint: Coordinator's receive_result method).

3. Design a solution for writing files to disk. Consider:
   - Should we add a FileWriterAgent, or a file_writer utility module?
   - What directory structure should be created? (Suggestion: timestamped like `agent-team-output-YYYY-MM-DD-HH-MM-SS/`)
   - What files should always be created? (Minimum: Cargo.toml + src/main.rs)
   - Where in the pipeline should file writing happen? (Suggestion: after validation passes, in Coordinator)

4. Return a detailed architecture plan with:
   - Which files to create or modify
   - What each file will contain (function signatures, main logic)
   - Where file writing is triggered
   - How the Coordinator reports the output directory path to the user

Format as a numbered plan that Agents 2, 3, and 4 can execute.
```

**Wait for Agent 1 to finish before proceeding.**

---

### Agent 2 (Sonnet 4.5) — Implementer

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"`.

**Prompt to pass (include Agent 1's plan):**

```
You are the Implementer agent fixing the agent-team project to write real files to disk.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Implement the file-writing system designed by the Architect.

**Architect's Plan:**

[INSERT AGENT 1 OUTPUT HERE]

**Steps:**

1. Based on the Architect's plan, create or modify the necessary files. For example:
   - If the plan says "create src/file_writer.rs", create it with the specified functions
   - If the plan says "create src/agents/file_writer.rs", create a new agent module

2. Implement the core file-writing functions:
   - Function to create a timestamped directory (e.g., `agent-team-output-2026-02-18-14-23-05/`)
   - Function to write Cargo.toml with the correct project name and metadata
   - Function to create src/ directory
   - Function to write src/main.rs with the generated code
   - Function that ties it all together (takes task description + code, writes all files, returns directory path)

3. Add any needed dependencies to Cargo.toml (likely none — std::fs should be sufficient).

4. Run `cargo check` from /workspace/jashan/agent_developer_project to verify no compilation errors.

5. Report: which files you created/modified, key functions implemented, and cargo check result.
```

**Wait for Agent 2 to finish before proceeding.**

---

### Agent 3 (Sonnet 4.5) — Integrator

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"`.

**Prompt to pass (include Agent 1's plan and Agent 2's report):**

```
You are the Integrator agent fixing the agent-team project to write real files to disk.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Wire the file-writing system into the Pipeline and Coordinator.

**Architect's Plan:**

[INSERT AGENT 1 OUTPUT HERE]

**Implementer's Report:**

[INSERT AGENT 2 OUTPUT HERE]

**Steps:**

1. Read src/agents/coordinator.rs and locate the `receive_result` method.

2. Modify `receive_result` to:
   - Call the file writer after validation passes
   - Pass the task description and final code to the file writer
   - Capture the output directory path returned by the file writer
   - Print the directory path in the final output (e.g., "Project created at: ./agent-team-output-2026-02-18-14-23-05/")

3. If the file writer is a module (not an agent), add `mod file_writer;` to src/main.rs and make sure it's properly imported in coordinator.rs.

4. If the file writer is a new agent, add it to src/agents/mod.rs and update the Pipeline to include it.

5. Update src/messages.rs if any new message types are needed (likely not, but check the Architect's plan).

6. Run `cargo build` from /workspace/jashan/agent_developer_project. If it fails, fix the errors and rebuild.

7. Report: which files you modified, what changes you made to Coordinator/Pipeline, and build result.
```

**Wait for Agent 3 to finish before proceeding.**

---

### Agent 4 (Sonnet 4.5) — Validator

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"`.

**Prompt to pass:**

```
You are the Validator agent fixing the agent-team project to write real files to disk.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Test the file-writing functionality end-to-end, then commit.

**Steps:**

1. Run `cargo build` from /workspace/jashan/agent_developer_project — must succeed.

2. Run the agent-team interactively with a test task:
   ```bash
   cd /workspace/jashan/agent_developer_project
   echo "write a function that adds two numbers" | cargo run
   ```
   Let it run and capture the output.

3. Check if a new directory was created (should be named like `agent-team-output-YYYY-MM-DD-HH-MM-SS/`):
   ```bash
   ls -la /workspace/jashan/agent_developer_project/ | grep agent-team-output
   ```

4. If the directory exists, check its contents:
   ```bash
   ls -R /workspace/jashan/agent_developer_project/agent-team-output-*/
   cat /workspace/jashan/agent_developer_project/agent-team-output-*/Cargo.toml
   cat /workspace/jashan/agent_developer_project/agent-team-output-*/src/main.rs
   ```

5. Verify the created project is runnable:
   ```bash
   cd /workspace/jashan/agent_developer_project/agent-team-output-*/
   cargo build
   cargo run
   ```
   Capture and display the output. It should run without errors.

6. If all checks pass, commit all changes:
   ```bash
   git -C /workspace/jashan/agent_developer_project add -A
   git -C /workspace/jashan/agent_developer_project commit -m "feat: agent-team now writes real project files to disk instead of just printing code

   - Added file-writing system (architect + implementer work)
   - Coordinator now creates timestamped output directories
   - Generated projects are immediately runnable with cargo run
   - Includes Cargo.toml + src/main.rs with all generated code

   Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
   ```

7. Report: build status, whether the directory was created, contents of Cargo.toml and src/main.rs, whether cargo run succeeded in the generated project, and the git commit hash.
```

---

## After All 4 Agents Finish

Report to the user:
- Which files were created or modified
- Confirmation that agent-team now writes files to disk
- Example of the directory structure created
- The git commit hash
- Next steps: run `bash scripts/install.sh` to update the installed binary, then test it with `agent-team` from any directory

Done.
```

---

### Step 2: Update CLAUDE.md Commands Section

Add the new `/agent-fix` command to the Commands section of CLAUDE.md.

**Actions:**

- Open CLAUDE.md
- Locate the Commands section
- Add a new entry for `/agent-fix` after `/refactor`

**Files affected:**

- `CLAUDE.md`

**Content to add:**

```markdown
### /agent-fix

**Purpose:** Fix the agent-team binary to write real project files to disk instead of just printing code to stdout.

Run this when you notice that agent-team generates code but doesn't create any files. Claude will spawn 4 agents who:

1. Architect agent reads the codebase and designs a file-writing system
2. Implementer agent creates the file-writing module or agent
3. Integrator agent wires file writing into the Coordinator
4. Validator agent tests end-to-end and commits

After running: The agent-team will create timestamped project directories (e.g., `agent-team-output-2026-02-18-14-23-05/`) with Cargo.toml, src/main.rs, and all generated code. You can immediately cd into the directory and run `cargo run`.
```

---

### Step 3: Validate the Command File

Run a quick check to ensure the command file is correctly formatted and executable.

**Actions:**

- Read `.claude/commands/agent-fix.md` to verify content
- Check that it follows the same structure as other command files (e.g., `/refactor`, `/brain-chips`)
- Confirm all 4 agent prompts are complete and have clear instructions

**Files affected:**

- `.claude/commands/agent-fix.md` (read-only check)

---

### Step 4: Commit the Command Creation

Commit the new `/agent-fix` command to git.

**Actions:**

- Stage `.claude/commands/agent-fix.md` and `CLAUDE.md`
- Write a descriptive commit message
- Commit

**Files affected:**

- `.claude/commands/agent-fix.md`
- `CLAUDE.md`

**Commit command:**

```bash
git add .claude/commands/agent-fix.md CLAUDE.md
git commit -m "feat: add /agent-fix command — fixes agent-team to write real project files

- New command spawns 4 agents to add file-writing capability
- Architect designs, Implementer builds, Integrator wires, Validator tests
- After fix: agent-team creates timestamped directories with runnable Rust projects
- Updated CLAUDE.md Commands section

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — lists all available commands, will be updated to include `/agent-fix`
- `.claude/commands/` — directory containing all command files

### Updates Needed for Consistency

- `CLAUDE.md` Commands section must list `/agent-fix` with a clear description
- No other files need updates — this is a new command, not a modification of existing workflows

### Impact on Existing Workflows

- **None** — this command is additive. Existing commands like `/refactor`, `/brain-chips`, `/fix` are unaffected.
- After running `/agent-fix` and rebuilding the binary, the agent-team's behavior will change: it will start writing files to disk. This is the intended effect and improves usability.

---

## Validation Checklist

How to verify the implementation is complete and correct:

- [ ] `.claude/commands/agent-fix.md` exists and contains 4 complete agent specifications
- [ ] `CLAUDE.md` Commands section includes `/agent-fix` entry
- [ ] All 4 agent prompts in the command file are clear, detailed, and executable
- [ ] Agent prompts specify the correct working directory (`/workspace/jashan/agent_developer_project`)
- [ ] The command follows the established pattern from `/refactor` and `/brain-chips`
- [ ] Git commit created with descriptive message
- [ ] Running `/agent-fix` in a future Claude Code session will spawn the 4 agents as specified

---

## Success Criteria

The implementation is complete when:

1. The file `.claude/commands/agent-fix.md` exists and contains a complete, executable command specification with 4 agent prompts.

2. `CLAUDE.md` lists `/agent-fix` in the Commands section with a clear description of what it does.

3. The command file follows the exact format of existing multi-agent commands (like `/refactor`).

4. A git commit has been created documenting the new command.

5. A future Claude Code session can successfully invoke `/agent-fix` via the Skill tool, and the 4 agents will spawn in sequence to implement file-writing functionality.

---

## Notes

- **This plan creates the command, not the fix itself**: The `/agent-fix` command, when run, will spawn agents who implement the actual fix. This plan is about creating that command.

- **Agents have full autonomy**: The agent prompts are intentionally flexible. They specify goals and constraints but let the agents make architectural decisions (e.g., FileWriterAgent vs file_writer module, exact directory naming scheme, etc.).

- **Timestamped directories prevent conflicts**: Using `agent-team-output-YYYY-MM-DD-HH-MM-SS/` as the output directory pattern means multiple runs won't overwrite each other.

- **Future enhancements** (not in this plan):
  - Let the user specify the output directory with a flag like `agent-team --output my-project/`
  - Support multi-file projects (e.g., lib.rs + main.rs, or multiple modules)
  - Add a `--no-write` flag to print code without writing files (for quick tests)

- **Compatibility with `/make-path-interactive`**: After running `/agent-fix` and the agents implement the fix, users will need to run `bash scripts/install.sh` (or `/make-path-interactive`) to update the installed binary in `~/.local/bin/`.

---
