# Plan: Make Installed Binary Interactive by Default

**Created:** 2026-02-17
**Status:** Implemented
**Request:** Create a `/make-path-interactive` command that rebuilds and reinstalls the agent-team binary so the installed version (`agent-team` run from PATH) launches in interactive mode by default, just like `cargo run` does.

---

## Overview

### What This Plan Accomplishes

This plan creates a new slash command `/make-path-interactive` that rebuilds the release binary and reinstalls it to `~/.local/bin/`. Running this command ensures the installed `agent-team` binary is always in sync with the current source code, which already has interactive mode as its default behavior.

### Why This Matters

When you run `cargo run`, you're running the latest compiled debug build from source — interactive mode is the default. But if the installed binary at `~/.local/bin/agent-team` was built from an older version of the code, it may behave differently. This command gives a single, beginner-friendly way to "push" the current interactive-mode behavior into the installed binary.

---

## Current State

### Relevant Existing Structure

- `src/main.rs` — already has interactive mode as the default; `main()` calls `interactive_loop()` directly
- `scripts/install.sh` — existing install script that builds release binary and copies to `~/.local/bin/`
- `~/.local/bin/agent-team` — installed binary already exists (last installed 2026-02-17)
- `.claude/commands/add-installable-path.md` — original command that created the install script
- `.claude/commands/interactive-mode.md` — command to run interactive mode via `cargo run`

### Gaps or Problems Being Addressed

There is no single command dedicated to "sync the installed binary with the current code." A user who runs `agent-team` from their terminal might be running a stale binary that doesn't reflect the latest source. This command fixes that gap — one command, always up to date.

---

## Proposed Changes

### Summary of Changes

- Create a new slash command file `.claude/commands/make-path-interactive.md`
- Update `CLAUDE.md` to document the new `/make-path-interactive` command

### New Files to Create

| File Path | Purpose |
| --- | --- |
| `.claude/commands/make-path-interactive.md` | Slash command that rebuilds and reinstalls the binary so the PATH version runs interactive mode by default |

### Files to Modify

| File Path | Changes |
| --- | --- |
| `CLAUDE.md` | Add `/make-path-interactive` to the Commands section |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **Reuse `scripts/install.sh` rather than duplicating logic**: The install script already does exactly what is needed — build release binary, copy to `~/.local/bin/`. The new command simply calls that script instead of reinventing the steps.

2. **Name the command `/make-path-interactive`**: The name directly matches what the user asked for and clearly describes the intent: make the PATH-installed version interactive.

3. **Verify behavior after install**: The command verifies the installed binary runs and prints the interactive mode prompt so the user gets clear confirmation it worked.

4. **No changes to Rust source needed**: `src/main.rs` already defaults to interactive mode. The problem is purely that the installed binary may be out of date — so we just need to rebuild and reinstall.

### Alternatives Considered

- **Modify `src/main.rs`**: Not needed — interactive mode is already the default. The source is correct; only the installed binary needs refreshing.
- **Create a new install script**: Not needed — `scripts/install.sh` already handles this cleanly. Reusing it keeps things simple.

### Open Questions (if any)

None — the approach is clear and all prerequisites exist.

---

## Step-by-Step Tasks

### Step 1: Create the `/make-path-interactive` command file

Create a new command file at `.claude/commands/make-path-interactive.md` that instructs Claude to rebuild the release binary and reinstall it to `~/.local/bin/`.

**Actions:**

- Create `.claude/commands/make-path-interactive.md` with the full command content below

**Full file content:**

```markdown
# Make Path Interactive

Rebuild the agent-team release binary and reinstall it to `~/.local/bin/` so that
running `agent-team` from anywhere in the terminal launches interactive mode by default —
the same as running `cargo run` from the project directory.

---

## Instructions

### Phase 1: Rebuild the Release Binary

Run the install script which rebuilds and reinstalls in one step:

```bash
bash scripts/install.sh
```

If the script succeeds, the binary at `~/.local/bin/agent-team` is now up to date.
If `cargo build --release` fails, read the error output and fix the compilation error before continuing.

---

### Phase 2: Verify Interactive Mode Is Active

Run the installed binary directly by path to confirm it launches interactive mode:

```bash
echo "exit" | ~/.local/bin/agent-team
```

The output should contain the text `INTERACTIVE MODE ACTIVE`.

If `~/.local/bin` is in PATH, also test:

```bash
echo "exit" | agent-team
```

---

### Phase 3: Report

After completion, tell the user:

1. **Build result:** whether `cargo build --release` succeeded
2. **Install location:** `~/.local/bin/agent-team`
3. **Verification:** whether the binary printed `INTERACTIVE MODE ACTIVE`
4. **How to use:** run `agent-team` from any terminal — it will launch in interactive mode
```

**Files affected:**

- `.claude/commands/make-path-interactive.md` (new file)

---

### Step 2: Update CLAUDE.md

Add the new `/make-path-interactive` command to the Commands section in `CLAUDE.md`, following the same format as the other commands.

**Actions:**

- Open `CLAUDE.md`
- Find the Commands section
- Add a new entry for `/make-path-interactive` after the `/interactive-mode` entry

**Content to add** (in the Commands section table and as a new `### /make-path-interactive` subsection):

In the Commands table inside the workspace structure section:

```
| `/make-path-interactive` | Rebuild and reinstall the binary so `agent-team` runs interactive mode from PATH |
```

In the Commands documentation section, add after the `/interactive-mode` entry:

```markdown
### /make-path-interactive

**Purpose:** Rebuild and reinstall the agent-team binary so the installed PATH version launches interactive mode by default.

Run this whenever you've made changes to the source code and want the installed `agent-team` binary to reflect those changes. Claude will:

1. Run `bash scripts/install.sh` to rebuild the release binary
2. Copy the updated binary to `~/.local/bin/agent-team`
3. Verify the installed binary launches in interactive mode
4. Report the result

Use this to keep the installed binary in sync with the source code.
```

**Files affected:**

- `CLAUDE.md`

---

### Step 3: Commit the changes

Stage and commit all new and modified files.

**Actions:**

- Stage the new command file and updated CLAUDE.md
- Commit with a descriptive message

```bash
git add .claude/commands/make-path-interactive.md CLAUDE.md
git commit -m "feat: add /make-path-interactive command to sync installed binary"
```

**Files affected:**

- `.claude/commands/make-path-interactive.md`
- `CLAUDE.md`

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — lists all commands; needs the new entry
- `scripts/install.sh` — reused by the new command; must already exist (it does)
- `src/main.rs` — source of the interactive behavior; already correct, no changes needed

### Updates Needed for Consistency

- `CLAUDE.md` must be updated so future `/prime` sessions know about this command

### Impact on Existing Workflows

- No existing workflows are changed
- The new command is additive — it complements `/add-installable-path` (which installs for the first time) and `/interactive-mode` (which runs via `cargo run`)

---

## Validation Checklist

- [ ] `.claude/commands/make-path-interactive.md` exists and contains complete instructions
- [ ] Running `/make-path-interactive` causes Claude to run `bash scripts/install.sh`
- [ ] After the command runs, `echo "exit" | ~/.local/bin/agent-team` prints `INTERACTIVE MODE ACTIVE`
- [ ] `CLAUDE.md` lists `/make-path-interactive` in the Commands section
- [ ] Changes are committed to git

---

## Success Criteria

The implementation is complete when:

1. `.claude/commands/make-path-interactive.md` exists with complete, runnable instructions
2. Running the command updates the installed binary and the binary confirms interactive mode is active
3. `CLAUDE.md` documents the command so it's discoverable in future sessions

---

## Notes

- The Rust source in `src/main.rs` already defaults to interactive mode — `main()` calls `interactive_loop()` directly with no flags needed. No source changes are required.
- The install script at `scripts/install.sh` already handles the full build + copy workflow. This command simply provides a dedicated, named entry point for "re-sync the installed binary."
- If in the future the source changes and interactive mode becomes non-default again, this command will need to be updated to pass the appropriate flag.

---

## Implementation Notes

**Implemented:** 2026-02-17

### Summary

- Created `.claude/commands/make-path-interactive.md` with full Phase 1–3 instructions
- Updated `CLAUDE.md` to add the new command to the file tree and the Commands documentation section
- Committed both files to git

### Deviations from Plan

The new command file had to be force-added (`git add -f`) because `.claude/` is listed in `.gitignore`. This matches the pattern used by all other tracked command files in `.claude/commands/` (they were also previously force-added).

### Issues Encountered

None beyond the gitignore handling noted above.
