# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## What This Is

This is a **Claude Workspace Template** — a structured environment designed for working with Claude Code as a powerful agent assistant across sessions. The user will spin up fresh Claude Code sessions repeatedly, using `/prime` at the start of each to load essential context without bloat.

**This file (CLAUDE.md) is the foundation.** It is automatically loaded at the start of every session. Keep it current — it is the single source of truth for how Claude should understand and operate within this workspace.

---

## The Claude-User Relationship

Claude operates as an **agent assistant** with access to the workspace folders, context files, commands, and outputs. The relationship is:

- **User**: Defines goals, provides context about their role/function, and directs work through commands
- **Claude**: Reads context, understands the user's objectives, executes commands, produces outputs, and maintains workspace consistency

Claude should always orient itself through `/prime` at session start, then act with full awareness of who the user is, what they're trying to achieve, and how this workspace supports that.

---

## Workspace Structure

```
.
├── CLAUDE.md              # This file — core context, always loaded
├── .claude/
│   └── commands/          # Slash commands Claude can execute
│       ├── prime.md       # /prime — session initialization
│       ├── instructions.md # /instructions — install and run guidance
│       ├── create-plan.md  # /create-plan — create implementation plans
│       ├── implement.md   # /implement — execute plans
│       ├── template.md    # /template — scaffold files from plan
│       ├── finish.md      # /finish — fully implement all stubbed files
│       ├── project-testing.md  # /project-testing — test the running project against the plan
│       ├── interactive-mode.md # /interactive-mode — run agent team interactively
│       ├── make-path-interactive.md # /make-path-interactive — sync installed binary to latest source
│       └── fix.md             # /fix — fix agent bugs and overhaul interactive UI
├── context/               # Background context about the user and project
│                          # (User should populate with role, goals, strategies)
├── plans/                 # Implementation plans created by /create-plan
├── outputs/               # Work products and deliverables
├── reference/             # Templates, examples, reusable patterns
└── scripts/               # Automation scripts (if applicable)
```

**Key directories:**

| Directory    | Purpose                                                                             |
| ------------ | ----------------------------------------------------------------------------------- |
| `context/`   | Who the user is, their role, current priorities, strategies. Read by `/prime`.      |
| `plans/`     | Detailed implementation plans. Created by `/create-plan`, executed by `/implement`. |
| `outputs/`   | Deliverables, analyses, reports, and work products.                                 |
| `reference/` | Helpful docs, templates and patterns to assist in various workflows.                |
| `scripts/`   | Any automation or tooling scripts.                                                  |
| `src/`       | Rust source code for the multi-agent coding system                                  |
| `Cargo.toml` | Rust project manifest                                                               |
| `README.md`  | Beginner-friendly project documentation                                             |

---

## Commands

### /prime

**Purpose:** Initialize a new session with full context awareness.

Run this at the start of every session. Claude will:

1. Read CLAUDE.md and context files
2. Summarize understanding of the user, workspace, and goals
3. Confirm readiness to assist

### /instructions

**Purpose:** Display beginner-friendly instructions for how to install and run the agent-team project.

Run this when you want a quick guide to getting the project running. Claude will read the current project files and present clear step-by-step instructions covering prerequisites, building, running, and optional PATH installation.

### /create-plan [request]

**Purpose:** Create a detailed implementation plan before making changes.

Use when adding new functionality, commands, scripts, or making structural changes. Produces a thorough plan document in `plans/` that captures context, rationale, and step-by-step tasks.

Example: `/create-plan add a competitor analysis command`

### /implement [plan-path]

**Purpose:** Execute a plan created by /create-plan.

Reads the plan, executes each step in order, validates the work, and updates the plan status.

Example: `/implement plans/2026-01-28-competitor-analysis-command.md`

### /template

**Purpose:** Scaffold all files defined in the current project plan with starter template code.

Run this after `/create-plan` and before `/implement` to create the file structure. Claude will:

1. Find the oldest plan in `plans/`
2. Read the "New Files to Create" table
3. Create each file at its listed path with minimal starter code (stubs, not full implementation)
4. Report what was created

This gives you a navigable project skeleton before running `/implement` to fill in the full code.

### /finish

**Purpose:** Fully implement all stubbed files from the current project plan, producing a complete working project.

When invoked, Claude will:

1. Find the oldest plan in `plans/`
2. Read the complete file content specifications from the plan's Step-by-Step Tasks
3. Write every file completely (overwriting any stubs from `/template`)
4. Run `cargo build` to verify the project compiles
5. Commit all changes to git

Use this after `/template` has scaffolded the files and you're ready to go from stubs to a fully working project.

### /project-testing

**Purpose:** Verify the running project behaves exactly as its plan describes. Fixes failures automatically and only reports "done" when all behavioral expectations pass.

When invoked, Claude will:

1. Find the oldest plan in `plans/`
2. Extract behavioral expectations from the plan (what each agent should print, what order they run, what output the program produces)
3. Build the project with `cargo build`
4. Run the project with `cargo run` and capture the output
5. Check every expectation against the actual output
6. If anything fails, call a fix agent and re-run from scratch
7. Loop until all expectations pass (max 5 rounds), then report "done"

Use this after `/finish` to confirm the project actually runs the way the plan intended.

### /add-installable-path

**Purpose:** Build the agent-team release binary and install it to `~/.local/bin/` so it can be run as `agent-team` from any directory in the terminal.

When invoked, Claude will:

1. Create `scripts/install.sh` — a reusable install script
2. Run `cargo build --release` to build the optimized binary
3. Copy the binary to `~/.local/bin/agent-team`
4. Add `~/.local/bin` to `$PATH` in `~/.bashrc` / `~/.zshrc` if not already present
5. Verify the binary runs correctly
6. Update `README.md` with installation instructions
7. Append the installable-path feature to the oldest plan in `plans/`
8. Commit all changes

Use this once to make agent-team available system-wide.

### /interactive-mode

**Purpose:** Launch the agent team in interactive mode — type any coding task and the agents work on it live.

When invoked, Claude will:

1. Run `cargo build` to ensure the binary is up to date
2. Run `cargo run` to start the interactive session (interactive mode is the default)
3. The agent team prints a prompt and waits for your input
4. Type any coding task (e.g., "write a function that reverses a string") and press Enter
5. All five agents work on your task and print their output
6. Type another task to continue, or type `exit` to stop

Use this when you want to give the agent team your own tasks.

### /make-path-interactive

**Purpose:** Rebuild and reinstall the agent-team binary so the installed PATH version launches interactive mode by default.

Run this whenever you've made changes to the source code and want the installed `agent-team` binary to reflect those changes. Claude will:

1. Run `bash scripts/install.sh` to rebuild the release binary
2. Copy the updated binary to `~/.local/bin/agent-team`
3. Verify the installed binary launches in interactive mode
4. Report the result

Use this to keep the installed binary in sync with the source code.

### /fix

**Purpose:** Spawn multiple agents to fix the agent-team project — repair the bug where agents say "finished" but ignore the user's task, then overhaul the interactive mode UI to match modern CLI tools.

When invoked, Claude will:

1. Spawn parallel subagents to diagnose the task-mismatch bug in `src/agents/`
2. Fix the Coder and Planner to handle 15+ task types correctly
3. Add a ValidatorAgent that confirms output matches the task before reporting success
4. Overhaul `src/main.rs` with a styled UI (colored prompt, agent labels, dividers)
5. Run `cargo build` to verify the project compiles
6. Commit all changes

Use this when the agents are giving wrong/generic output or when the UI needs improvement.

### /brain-chips

**Purpose:** Wire each agent to a real AI brain — each agent calls a live AI model matched to its specialty.

When invoked, Claude will:

1. Add `reqwest`, `serde`, `serde_json` to `Cargo.toml`
2. Create `src/ai_client.rs` with HTTP functions for OpenAI, DeepSeek, and Anthropic
3. Rewrite all six agents to make real API calls to their assigned model
4. Run `cargo build` to verify compilation
5. Commit all changes

Brain assignments:
- **Coordinator + Planner** → Claude (Architecture)
- **Coder** → GPT-4o (Coding)
- **Reviewer** → Claude (Security & Docs)
- **Debugger** → DeepSeek-Coder (Debugging & Optimization)
- **Validator** → Claude (Testing)

Requires environment variables: `OPENAI_API_KEY`, `DEEPSEEK_API_KEY`, `ANTHROPIC_API_KEY`
### /refactor

**Purpose:** Delete ALL external AI API code and rewrite every agent with a specialized pure-Rust brain — zero API keys, zero external services needed.

When invoked, Claude will spawn 4 agents (1 Opus 4.5 + 3 Sonnet 4.5):
1. Opus 4.5 reads the codebase and plans the rewrite
2. Sonnet 4.5 deletes ai_client.rs, removes HTTP dependencies, cleans main.rs
3. Sonnet 4.5 rewrites all agent files with built-in Rust brains
4. Sonnet 4.5 builds, verifies, updates docs, commits

After running: **zero API keys needed**. Just `cargo run`.

Built-in brain assignments:
- **Planner** — keyword-based task analysis → tailored steps (sort, reverse, fibonacci, etc.)
- **Coder** — task-type detection → real working Rust code templates
- **Reviewer** — static analysis rules → code quality checks
- **Debugger** — known fix patterns → automatic issue resolution
- **Validator** — keyword + structure checks → task match confirmation



---

## Critical Instruction: Maintain This File

**Whenever Claude makes changes to the workspace, Claude MUST consider whether CLAUDE.md needs updating.**

After any change — adding commands, scripts, workflows, or modifying structure — ask:

1. Does this change add new functionality users need to know about?
2. Does it modify the workspace structure documented above?
3. Should a new command be listed?
4. Does context/ need new files to capture this?

If yes to any, update the relevant sections. This file must always reflect the current state of the workspace so future sessions have accurate context.

**Examples of changes requiring CLAUDE.md updates:**

- Adding a new slash command → add to Commands section
- Creating a new output type → document in Workspace Structure or create a section
- Adding a script → document its purpose and usage
- Changing workflow patterns → update relevant documentation

---

## For Users Downloading This Template

To customize this workspace to your own needs, fill in your context documents in `context/` and modify as needed. Then use `/create-plan` to plan out and `/implement` to execute any structural changes. This ensures everything stays in sync — especially CLAUDE.md, which must always reflect the current state of the workspace.

---

## Session Workflow

1. **Start**: Run `/prime` to load context
2. **Work**: Use commands or direct Claude with tasks
3. **Plan changes**: Use `/create-plan` before significant additions
4. **Scaffold (optional)**: Use `/template` to create starter files from the plan
5. **Finish**: Use `/finish` to fully implement all files from the plan (build + commit included)
6. **Test**: Use `/project-testing` to confirm the running project matches the plan's behavioral expectations
7. **Execute (alternative)**: Use `/implement` to execute plans step-by-step with a specific plan path
8. **Maintain**: Claude updates CLAUDE.md and context/ as the workspace evolves

---

## Notes

- Keep context minimal but sufficient — avoid bloat
- Plans live in `plans/` with dated filenames for history
- Outputs are organized by type/purpose in `outputs/`
- Reference materials go in `reference/` for reuse
