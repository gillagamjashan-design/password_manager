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
