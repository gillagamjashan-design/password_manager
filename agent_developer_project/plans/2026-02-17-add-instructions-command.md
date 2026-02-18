# Plan: Add /instructions Command

**Created:** 2026-02-17
**Status:** Draft
**Request:** Create a `/instructions` command that gives users step-by-step instructions on how to install and run the agent-team project.

---

## Overview

### What This Plan Accomplishes

This plan creates a new slash command `/instructions` that, when run, prints clear beginner-friendly instructions for how to install and run the agent-team project. The command reads the current project state and presents everything a new user needs to get started.

### Why This Matters

The project's core goal is to be beginner-friendly. New users cloning the repo or opening the workspace for the first time may not know where to start — this command gives them instant, context-aware guidance without needing to read the full README.

---

## Current State

### Relevant Existing Structure

- `.claude/commands/prime.md` — minimal command, just reads files and summarizes
- `.claude/commands/add-installable-path.md` — detailed phase-based command with bash steps
- `README.md` — contains full build, run, and install instructions already
- `scripts/install.sh` — install script for PATH setup
- `Cargo.toml` — Rust project config
- `src/` — Rust source files

### Gaps or Problems Being Addressed

There is no quick-access command a user can run inside Claude Code to get install and run instructions. A user would need to open `README.md` manually or already know the commands. This `/instructions` command fills that gap.

---

## Proposed Changes

### Summary of Changes

- Create `.claude/commands/instructions.md` — the new slash command
- Update `CLAUDE.md` — add `/instructions` to the Commands section

### New Files to Create

| File Path | Purpose |
| --- | --- |
| `.claude/commands/instructions.md` | New slash command that displays how to install and run the project |

### Files to Modify

| File Path | Changes |
| --- | --- |
| `CLAUDE.md` | Add `/instructions` entry to the Commands section |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **Output instructions directly in Claude's response, not via a script:** The command should have Claude read the README and relevant files, then present the instructions in a clear, formatted way — matching how `/prime` works. No bash script is needed since this is purely informational.

2. **Cover both run methods (cargo run and agent-team PATH):** The instructions should cover both ways to run the project so any user — whether they've installed to PATH or not — gets the right steps.

3. **Keep the command simple and read-only:** This command does not modify files or run builds. It only reads and presents information. This keeps it fast and safe to run at any time.

4. **Include prerequisites clearly:** Since this is a beginner-friendly project, the instructions should include the Rust installation prerequisite, not assume it's already done.

### Alternatives Considered

- **Run a bash script that prints instructions:** Rejected — Claude presenting formatted output is cleaner and more readable in the CLI than shell echo statements.
- **Just point the user to README.md:** Rejected — the whole point of a command is instant, summarized guidance without requiring the user to navigate files.

### Open Questions (if any)

None. All decisions are clear.

---

## Step-by-Step Tasks

Execute these tasks in order during implementation.

### Step 1: Create the /instructions command file

Create `.claude/commands/instructions.md` with full content for the command.

**Actions:**

- Create the file at `.claude/commands/instructions.md`
- The command should instruct Claude to:
  1. Read `README.md` for current install/run instructions
  2. Read `Cargo.toml` to confirm project name
  3. Check whether `scripts/install.sh` exists
  4. Present a clean, beginner-friendly set of instructions covering:
     - Prerequisites (Rust installation)
     - How to build with `cargo build`
     - How to run with `cargo run`
     - How to install to PATH with `bash scripts/install.sh`
     - How to run as a system command with `agent-team`

**File content to write:**

```markdown
# Instructions

> Give the user clear, beginner-friendly instructions for how to install and run this project.

## Read

- `README.md`
- `Cargo.toml`
- `scripts/install.sh` (if it exists)

## Instructions to Present

After reading the files above, present the following to the user in a clean, well-formatted way:

### 1. Prerequisites

Explain that Rust must be installed first. If the user doesn't have it:

- Go to https://rustup.rs
- Follow the instructions for their operating system
- Verify with: `rustc --version`

### 2. Get the Project

Tell the user to clone or download the project, then open a terminal in the project folder.

### 3. Build the Project

```bash
cargo build
```

Explain this compiles the Rust source code. Only needs to be done once (or after making code changes).

### 4. Run the Project

```bash
cargo run
```

Explain this runs the agent team. They will see all 5 agents working through 3 sample tasks, printing their progress to the terminal.

### 5. Install to PATH (Optional — Run from Anywhere)

If `scripts/install.sh` exists, tell the user they can install the binary system-wide:

```bash
bash scripts/install.sh
source ~/.bashrc
```

Then they can run from any directory:

```bash
agent-team
```

### 6. Adding Custom Tasks

Show the user how to add their own tasks in `src/main.rs`:

```rust
pipeline.run("write a function that checks if a string is a palindrome");
```

## Tone

- Be friendly and encouraging
- Use numbered steps so it's easy to follow
- Avoid technical jargon unless necessary — this project is for beginners
- Remind the user they can ask questions if anything is unclear
```

**Files affected:**

- `.claude/commands/instructions.md` (new file)

---

### Step 2: Update CLAUDE.md

Add the `/instructions` command to the Commands section of `CLAUDE.md`.

**Actions:**

- Open `CLAUDE.md`
- Find the Commands section listing all slash commands
- Add a new entry for `/instructions` after `/prime` (since it's a user-facing informational command that naturally pairs with session initialization)

**Before (in the Commands section):**
```
### /prime
**Purpose:** Initialize a new session with full context awareness.
...
### /create-plan [request]
```

**After:**
```
### /prime
**Purpose:** Initialize a new session with full context awareness.
...
### /instructions
**Purpose:** Display beginner-friendly instructions for how to install and run the agent-team project.

Run this when you want a quick guide to getting the project running. Claude will read the current project files and present clear step-by-step instructions covering prerequisites, building, running, and optional PATH installation.

### /create-plan [request]
```

Also update the Commands table in the Workspace Structure section of CLAUDE.md to include `/instructions`.

**Files affected:**

- `CLAUDE.md`

---

### Step 3: Commit all changes

Stage and commit the new command file and updated CLAUDE.md.

**Actions:**

- `git add .claude/commands/instructions.md CLAUDE.md`
- Commit with message: `feat: add /instructions command for install and run guidance`

**Files affected:**

- `.claude/commands/instructions.md`
- `CLAUDE.md`

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — lists all available commands; needs the new entry
- `README.md` — source of truth for install/run content the command will read

### Updates Needed for Consistency

- `CLAUDE.md` Commands section (Step 2 above)
- `CLAUDE.md` Session Workflow section can optionally mention that `/instructions` can be shared with new users

### Impact on Existing Workflows

No existing workflows are changed. This is a purely additive command.

---

## Validation Checklist

- [ ] `.claude/commands/instructions.md` exists and has correct content
- [ ] Running `/instructions` in Claude Code produces clear, formatted install and run steps
- [ ] `CLAUDE.md` Commands section lists `/instructions` with an accurate description
- [ ] No existing commands are broken or changed
- [ ] Git commit created with all modified files

---

## Success Criteria

The implementation is complete when:

1. `/instructions` can be run in Claude Code and returns a beginner-friendly, step-by-step guide to installing and running the project
2. The instructions are accurate (matching `README.md` and `scripts/install.sh`)
3. `CLAUDE.md` is updated to document the new command
4. All changes are committed to git

---

## Notes

- This command is read-only and safe to run at any time — it does not modify files or trigger builds
- As the project evolves (e.g., new install methods, new run options), update `.claude/commands/instructions.md` to stay in sync with `README.md`
- Future enhancement: the command could detect whether Rust is installed and tailor the instructions accordingly
