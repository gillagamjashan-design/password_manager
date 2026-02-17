# Plan: Add /add-installable-path Command

**Created:** 2026-02-17
**Status:** Implemented
**Request:** Create a /add-installable-path command that, when run in Claude Code, adds a feature to the codebase that makes the agent-team binary installable in the user's PATH so it can be run anywhere with `agent-team` in the terminal, then appends this feature to the oldest plan in plans/.

---

## Overview

### What This Plan Accomplishes

This plan creates a `/add-installable-path` Claude Code command that, when executed, builds the `agent-team` release binary and installs it into the user's PATH so the command `agent-team` works from any directory in the terminal. It also creates a reusable `scripts/install.sh` shell script for future installs, updates the README with installation instructions, and appends an installable-path feature section to the oldest plan file.

### Why This Matters

Right now the agent-team binary can only be run from inside the project directory with `cargo run`. Making it installable in PATH turns it into a real tool you can call from anywhere — opening the door to using it as a general-purpose coding assistant in any project, which is exactly the goal described in the project's mission.

---

## Current State

### Relevant Existing Structure

```
agent_developer_project/
├── Cargo.toml                    # Defines binary name "agent-team", path = "src/main.rs"
├── README.md                     # Has build/run instructions, no install instructions
├── src/                          # Rust source code
│   └── main.rs, agents/, etc.
├── scripts/                      # Empty — no automation scripts yet
├── plans/
│   ├── 2026-02-17-rust-multi-agent-coding-system.md  ← OLDEST plan
│   ├── 2026-02-17-add-template-command.md
│   ├── 2026-02-17-add-finish-command.md
│   └── 2026-02-17-add-project-testing-command.md
└── .claude/
    └── commands/                 # prime, create-plan, implement, template, finish, project-testing
```

The binary name is already defined as `agent-team` in `Cargo.toml`. No install infrastructure exists yet.

### Gaps or Problems Being Addressed

- No way to run `agent-team` from outside the project directory
- No install script or instructions
- README only covers `cargo run`, not system-wide installation
- `scripts/` directory is empty despite being documented in CLAUDE.md

---

## Proposed Changes

### Summary of Changes

- Create `.claude/commands/add-installable-path.md` — the new slash command
- Create `scripts/install.sh` — reusable install script
- Update `README.md` — add installation instructions section
- Update `CLAUDE.md` — add `/add-installable-path` to the Commands section
- Append installable-path feature section to the oldest plan (`2026-02-17-rust-multi-agent-coding-system.md`)
- Build release binary and install it to `~/.local/bin/`
- Commit all changes

### New Files to Create

| File Path | Purpose |
| --------- | ------- |
| `.claude/commands/add-installable-path.md` | The /add-installable-path slash command definition |
| `scripts/install.sh` | Shell script to build release binary and install to PATH |

### Files to Modify

| File Path | Changes |
| --------- | ------- |
| `README.md` | Add "Installing to PATH" section with instructions |
| `CLAUDE.md` | Add /add-installable-path to the Commands section |
| `plans/2026-02-17-rust-multi-agent-coding-system.md` | Append installable-path feature section at the end |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **Install to `~/.local/bin/` by default**: This is a user-level install that doesn't require `sudo`. It follows the XDG Base Directory spec and is the standard for user-installed binaries on Linux. If `~/.local/bin` doesn't exist, the script creates it.

2. **Check and patch PATH in shell config**: After copying the binary, the script checks whether `~/.local/bin` is in `$PATH`. If not, it appends the export line to `~/.bashrc` (and `~/.zshrc` if it exists). This ensures the command works immediately after sourcing the config.

3. **Separate `scripts/install.sh` from the command**: The slash command orchestrates everything, but the actual install logic lives in `scripts/install.sh`. This means users can also run `bash scripts/install.sh` directly without needing Claude Code — making the project more self-contained.

4. **Use `cargo build --release`**: Release builds are optimized and produce a smaller, faster binary — appropriate for a tool installed system-wide.

5. **Append to the oldest plan, not create a new one**: The request says to add the feature to the oldest plan file. This keeps the feature's documentation co-located with the original system design it extends.

### Alternatives Considered

- **`cargo install --path .`**: Installs to `~/.cargo/bin/` instead. Rejected because it requires Cargo to be on the user's PATH (which it usually is), but it's less predictable across environments and doesn't give us control over the install location. `~/.local/bin` is more universal.
- **Install to `/usr/local/bin/`**: Requires `sudo`. Rejected — a student project shouldn't require root access for installation.
- **Symlinking instead of copying**: More elegant but can break if the project directory moves. Copying the binary is simpler and more robust.

### Open Questions (if any)

None — the approach is unambiguous given the constraints.

---

## Step-by-Step Tasks

Execute these tasks in order during implementation.

### Step 1: Create the /add-installable-path Command File

Create the slash command definition at `.claude/commands/add-installable-path.md`. This is what Claude executes when the user runs `/add-installable-path` in Claude Code.

**Actions:**

- Create `.claude/commands/add-installable-path.md` with the full command instructions below

**Files affected:**

- `.claude/commands/add-installable-path.md`

**Full content for `.claude/commands/add-installable-path.md`:**

```markdown
# Add Installable Path

Make the agent-team binary installable so it can be run from anywhere in the terminal with the command `agent-team`.

---

## Instructions

### Phase 1: Create the Install Script

Create `scripts/install.sh` with the following content:

```bash
#!/usr/bin/env bash
# install.sh — installs the agent-team binary to ~/.local/bin/
# Run this from the project root: bash scripts/install.sh

set -e  # Exit immediately if any command fails

echo "[install] Building agent-team in release mode..."
cargo build --release

BINARY_SRC="./target/release/agent-team"
INSTALL_DIR="$HOME/.local/bin"
BINARY_DEST="$INSTALL_DIR/agent-team"

echo "[install] Creating install directory if it doesn't exist..."
mkdir -p "$INSTALL_DIR"

echo "[install] Copying binary to $BINARY_DEST ..."
cp "$BINARY_SRC" "$BINARY_DEST"
chmod +x "$BINARY_DEST"

echo "[install] Binary installed at: $BINARY_DEST"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "[install] $INSTALL_DIR is not in your PATH. Adding it now..."

    EXPORT_LINE='export PATH="$HOME/.local/bin:$PATH"'

    # Add to ~/.bashrc
    if [ -f "$HOME/.bashrc" ]; then
        echo "$EXPORT_LINE" >> "$HOME/.bashrc"
        echo "[install] Added to ~/.bashrc"
    fi

    # Add to ~/.zshrc if it exists
    if [ -f "$HOME/.zshrc" ]; then
        echo "$EXPORT_LINE" >> "$HOME/.zshrc"
        echo "[install] Added to ~/.zshrc"
    fi

    echo ""
    echo "[install] IMPORTANT: Run the following command to apply the PATH change:"
    echo "          source ~/.bashrc"
    echo "          (or open a new terminal)"
else
    echo "[install] $INSTALL_DIR is already in your PATH. No changes needed."
fi

echo ""
echo "[install] Done! You can now run 'agent-team' from anywhere."
```

---

### Phase 2: Run the Install Script

1. Make the script executable and run it:
   ```bash
   chmod +x scripts/install.sh
   bash scripts/install.sh
   ```
2. Capture the output and confirm the binary was installed successfully.
3. If `cargo build --release` fails, read the error and fix it before continuing.

---

### Phase 3: Verify the Installation

1. Run the following to confirm the binary exists at the install location:
   ```bash
   ls -la ~/.local/bin/agent-team
   ```
2. Run the binary directly by path to confirm it works:
   ```bash
   ~/.local/bin/agent-team
   ```
3. If `~/.local/bin` is already in PATH, also test:
   ```bash
   agent-team
   ```

---

### Phase 4: Update the README

Add an "Installing to PATH" section to `README.md` after the "How to Build and Run" section:

```markdown
## Installing to PATH (Run from Anywhere)

To install `agent-team` as a system command you can run from any directory:

**Step 1: Run the install script**
```bash
bash scripts/install.sh
```

**Step 2: Apply the PATH change (if prompted)**
```bash
source ~/.bashrc
```

**Step 3: Run from anywhere**
```bash
agent-team
```

To reinstall after making code changes, just run `bash scripts/install.sh` again.
```

---

### Phase 5: Update the Oldest Plan

1. List all `.md` files in `plans/` sorted by name (earliest date = oldest).
2. Open the oldest plan file.
3. Append the following section at the very end of the file:

```markdown
---

## Feature Added: Installable PATH Support

**Added:** <today's date>

### What Was Added

The agent-team binary can now be installed to `~/.local/bin/` and run from anywhere in the terminal with the command `agent-team`.

### New Files

| File | Purpose |
|------|---------|
| `scripts/install.sh` | Builds release binary and installs it to `~/.local/bin/` |

### How to Install

```bash
bash scripts/install.sh
source ~/.bashrc   # if prompted
agent-team         # run from anywhere
```

### Implementation Notes

- Binary is installed to `~/.local/bin/agent-team`
- PATH is automatically updated in `~/.bashrc` and `~/.zshrc` if needed
- To reinstall after code changes, run `bash scripts/install.sh` again
```

---

### Phase 6: Commit All Changes

Stage and commit all modified and new files:

```
git add scripts/install.sh README.md plans/<oldest-plan-file>.md .claude/commands/add-installable-path.md CLAUDE.md
git commit -m "feat: add installable PATH support for agent-team binary"
```

---

## Report

After completion, provide:

1. **Install location:** confirm path where binary was installed
2. **PATH status:** whether PATH was already configured or was updated
3. **Verification:** output of running the binary directly
4. **Files modified:** list all files that were changed
5. **How to use:** remind user to run `source ~/.bashrc` if PATH was updated, then `agent-team`
```

---

### Step 2: Create scripts/install.sh

Create the install shell script at `scripts/install.sh`.

**Actions:**

- Create `scripts/install.sh` with the content specified in Phase 1 of the command above

**Files affected:**

- `scripts/install.sh`

**Full content for `scripts/install.sh`:**

```bash
#!/usr/bin/env bash
# install.sh — installs the agent-team binary to ~/.local/bin/
# Run this from the project root: bash scripts/install.sh

set -e  # Exit immediately if any command fails

echo "[install] Building agent-team in release mode..."
cargo build --release

BINARY_SRC="./target/release/agent-team"
INSTALL_DIR="$HOME/.local/bin"
BINARY_DEST="$INSTALL_DIR/agent-team"

echo "[install] Creating install directory if it doesn't exist..."
mkdir -p "$INSTALL_DIR"

echo "[install] Copying binary to $BINARY_DEST ..."
cp "$BINARY_SRC" "$BINARY_DEST"
chmod +x "$BINARY_DEST"

echo "[install] Binary installed at: $BINARY_DEST"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "[install] $INSTALL_DIR is not in your PATH. Adding it now..."

    EXPORT_LINE='export PATH="$HOME/.local/bin:$PATH"'

    # Add to ~/.bashrc
    if [ -f "$HOME/.bashrc" ]; then
        echo "$EXPORT_LINE" >> "$HOME/.bashrc"
        echo "[install] Added to ~/.bashrc"
    fi

    # Add to ~/.zshrc if it exists
    if [ -f "$HOME/.zshrc" ]; then
        echo "$EXPORT_LINE" >> "$HOME/.zshrc"
        echo "[install] Added to ~/.zshrc"
    fi

    echo ""
    echo "[install] IMPORTANT: Run the following command to apply the PATH change:"
    echo "          source ~/.bashrc"
    echo "          (or open a new terminal)"
else
    echo "[install] $INSTALL_DIR is already in your PATH. No changes needed."
fi

echo ""
echo "[install] Done! You can now run 'agent-team' from anywhere."
```

---

### Step 3: Update README.md

Add an "Installing to PATH" section after the "How to Build and Run" section.

**Actions:**

- Open `README.md`
- After the "How to Build and Run" section and before the "How to Add Your Own Tasks" section, insert the new section

**Files affected:**

- `README.md`

**New section to insert:**

```markdown
## Installing to PATH (Run from Anywhere)

To install `agent-team` as a system command you can run from any directory:

**Step 1: Run the install script**
```bash
bash scripts/install.sh
```

**Step 2: Apply the PATH change (if prompted)**
```bash
source ~/.bashrc
```

**Step 3: Run from anywhere**
```bash
agent-team
```

To reinstall after making code changes, just run `bash scripts/install.sh` again.
```

---

### Step 4: Update CLAUDE.md

Add `/add-installable-path` to the Commands section.

**Actions:**

- Open `CLAUDE.md`
- In the Commands section, add a new `### /add-installable-path` subsection after `/project-testing`

**Files affected:**

- `CLAUDE.md`

**New subsection to add:**

```markdown
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
```

---

### Step 5: Append Feature to Oldest Plan

Append the installable-path feature section to `plans/2026-02-17-rust-multi-agent-coding-system.md` (the oldest plan).

**Actions:**

- Open `plans/2026-02-17-rust-multi-agent-coding-system.md`
- Append the following section at the very end of the file

**Files affected:**

- `plans/2026-02-17-rust-multi-agent-coding-system.md`

**Content to append:**

```markdown
---

## Feature Added: Installable PATH Support

**Added:** 2026-02-17

### What Was Added

The agent-team binary can now be installed to `~/.local/bin/` and run from anywhere in the terminal with the command `agent-team`.

### New Files

| File | Purpose |
|------|---------|
| `scripts/install.sh` | Builds release binary and installs it to `~/.local/bin/` |

### How to Install

```bash
bash scripts/install.sh
source ~/.bashrc   # if prompted
agent-team         # run from anywhere
```

### Implementation Notes

- Binary is installed to `~/.local/bin/agent-team`
- PATH is automatically updated in `~/.bashrc` and `~/.zshrc` if needed
- To reinstall after code changes, run `bash scripts/install.sh` again
```

---

### Step 6: Build and Install the Binary

Execute the install process.

**Actions:**

- Run `cargo build --release` to build the optimized binary
- Run `bash scripts/install.sh` to install it
- Verify with `ls -la ~/.local/bin/agent-team`
- Run `~/.local/bin/agent-team` to confirm it executes correctly

**Files affected:**

- `target/release/agent-team` (built artifact)
- `~/.local/bin/agent-team` (installed binary)
- `~/.bashrc` (possibly updated with PATH export)

---

### Step 7: Commit All Changes

**Actions:**

- Stage all new and modified files by name (not `git add .`)
- Commit with a descriptive message

**Commit command:**
```bash
git add scripts/install.sh README.md CLAUDE.md plans/2026-02-17-rust-multi-agent-coding-system.md .claude/commands/add-installable-path.md
git commit -m "feat: add installable PATH support for agent-team binary

Adds /add-installable-path command, scripts/install.sh install script,
README install instructions, and installable-path feature section to
the original multi-agent system plan."
```

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — Commands section needs the new command listed
- `README.md` — needs installation instructions added
- `plans/2026-02-17-rust-multi-agent-coding-system.md` — oldest plan, gets the feature appended
- `Cargo.toml` — defines the binary name `agent-team` used in the install path

### Updates Needed for Consistency

- `CLAUDE.md` Commands section must include `/add-installable-path`
- `README.md` must include the new "Installing to PATH" section

### Impact on Existing Workflows

- `/prime` will load context that includes the new command
- After installation, users can run `agent-team` from any directory without needing to be in the project root
- The install script is reusable — running it again after code changes reinstalls the latest binary

---

## Validation Checklist

- [ ] `.claude/commands/add-installable-path.md` exists and has all 6 phases
- [ ] `scripts/install.sh` exists and is runnable with `bash scripts/install.sh`
- [ ] `cargo build --release` succeeds
- [ ] `~/.local/bin/agent-team` binary exists and is executable
- [ ] Running `~/.local/bin/agent-team` produces agent output
- [ ] `README.md` has the "Installing to PATH" section
- [ ] `CLAUDE.md` lists `/add-installable-path` in the Commands section
- [ ] `plans/2026-02-17-rust-multi-agent-coding-system.md` has the appended feature section
- [ ] All changes committed to git

---

## Success Criteria

The implementation is complete when:

1. `bash scripts/install.sh` runs without errors and copies the binary to `~/.local/bin/agent-team`
2. Running `~/.local/bin/agent-team` (or `agent-team` if PATH is set) produces the full agent-team output
3. The oldest plan file contains the appended "Feature Added: Installable PATH Support" section
4. All changes are committed to git

---

## Notes

- The binary name `agent-team` is already defined in `Cargo.toml` under `[[bin]] name = "agent-team"` — no Cargo.toml changes needed.
- `~/.local/bin` is preferred over `~/.cargo/bin` because it doesn't depend on Cargo being installed and is the standard XDG user binary location.
- If the user is on macOS instead of Linux, `~/.local/bin` still works but `~/.zshrc` is the primary shell config. The script handles both.
- Future enhancement: add an `uninstall.sh` script that removes the binary from `~/.local/bin`.

---

## Implementation Notes

**Implemented:** 2026-02-17

### Summary

All steps executed in order. Created `.claude/commands/add-installable-path.md` (the slash command), `scripts/install.sh` (the install script), updated `README.md` with installation instructions, updated `CLAUDE.md` with the new command, and appended the installable-path feature section to the oldest plan. Built the release binary with `cargo build --release` and installed it to `~/.local/bin/agent-team`. The command `agent-team` now works from any directory in the terminal.

### Deviations from Plan

None. All steps executed exactly as specified.

### Issues Encountered

None. `cargo build --release` succeeded on first attempt (2 non-blocking dead code warnings, same as before). `~/.local/bin` was already in the user's PATH, so no shell config modification was needed.
