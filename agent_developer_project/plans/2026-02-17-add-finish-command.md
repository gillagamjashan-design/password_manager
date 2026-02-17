# Plan: Add /finish Command

**Created:** 2026-02-17
**Status:** Implemented
**Request:** Create a `/finish` command that finds the oldest plan, reads it, and fully implements all template/stub files with complete final code — turning the scaffolded project into a working, finished project.

---

## Overview

### What This Plan Accomplishes

This plan creates a new slash command `/finish` that Claude can run in any session. When invoked, `/finish` finds the oldest plan in `plans/`, reads all the implementation details from it, and then writes the complete final code into every file that currently exists as a stub or template — replacing all `// TODO` placeholders with working implementations that match the plan's full Step-by-Step Tasks content.

### Why This Matters

The current workflow has three steps: `/create-plan` → `/template` (stubs) → `/implement` (full code). The `/finish` command collapses the last step into a self-contained command that any session can run without needing to know the plan path — it finds the plan automatically, reads the complete file specifications from the plan's Step-by-Step Tasks section, and writes everything out fully. This makes the workflow more beginner-friendly and session-independent.

---

## Current State

### Relevant Existing Structure

```
.claude/commands/
├── prime.md        # /prime — session init
├── create-plan.md  # /create-plan — plan creation
├── implement.md    # /implement — plan execution (requires plan path argument)
└── template.md     # /template — scaffolds stubs from plan

plans/
├── 2026-02-17-add-template-command.md        # older plan
└── 2026-02-17-rust-multi-agent-coding-system.md  # main project plan (has full file contents)

src/
├── main.rs          # STUB — has TODO, needs full code
├── messages.rs      # STUB — has TODO, needs full code
├── pipeline.rs      # STUB — has TODO, needs full code
├── task.rs          # STUB — has TODO, needs full code
└── agents/
    ├── mod.rs          # STUB — needs full code
    ├── coordinator.rs  # STUB — needs full code
    ├── planner.rs      # STUB — needs full code
    ├── coder.rs        # STUB — needs full code
    ├── reviewer.rs     # STUB — needs full code
    └── debugger.rs     # STUB — needs full code
```

### Gaps or Problems Being Addressed

- `/implement` requires the user to supply the plan path as an argument, which breaks if the user forgets the path or is in a fresh session
- No command exists that says "take the existing plan and finish all the stubbed files completely"
- The stub files from `/template` are not runnable — `cargo build` will fail until the TODOs are filled in
- A beginner needs a single command they can type to go from stubs → finished working project

---

## Proposed Changes

### Summary of Changes

- Create `.claude/commands/finish.md` — the new `/finish` command definition
- Update `CLAUDE.md` to document `/finish` in the Commands section

### New Files to Create

| File Path | Purpose |
| --------- | ------- |
| `.claude/commands/finish.md` | Defines the `/finish` command: finds oldest plan, reads full file content specs from Step-by-Step Tasks, writes every file completely |

### Files to Modify

| File Path | Changes |
| --------- | ------- |
| `CLAUDE.md` | Add `/finish` to the Commands section with purpose description |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **Auto-detect the oldest plan (same as `/template`)**: The `/finish` command uses the same plan-discovery logic as `/template` — find the oldest `.md` in `plans/`. This is consistent and means no argument is needed.

2. **Read file content from Step-by-Step Tasks, not New Files to Create table**: The "New Files to Create" table only has file paths and one-line descriptions. The full, complete file content is embedded in the Step-by-Step Tasks section under each step's content blocks. `/finish` reads those code blocks and writes them verbatim.

3. **Write files completely, not as stubs**: Unlike `/template` which writes `// TODO` stubs, `/finish` writes the full, final, production-ready code exactly as specified in the plan. Every file must be completely implemented — no TODOs left behind.

4. **Also create any missing files, not just overwrite stubs**: If a file from the plan doesn't exist yet (e.g., `README.md`, `Cargo.toml`), `/finish` creates it. If it exists as a stub, it overwrites it with the complete version.

5. **Run `cargo build` after writing all files**: After all files are written, `/finish` runs `cargo build` to verify the project compiles. If it fails, it reads the errors and fixes them before reporting success.

6. **Commit after successful build**: Per the user's requirement in `context/business-info.md`, changes should be committed. `/finish` commits all files after the build passes.

7. **Update the plan status to "Implemented"**: After successful completion, `/finish` updates the plan file's `**Status:**` field to `Implemented` and appends an Implementation Notes section — consistent with how `/implement` does it.

### Alternatives Considered

- **Making `/finish` an alias for `/implement` with auto-detected path**: Rejected — `/implement` has different phases and reporting structure; `/finish` has a cleaner, more focused purpose.
- **Requiring a plan path argument**: Rejected — the whole point is that the user shouldn't need to know or remember the path.
- **Not running `cargo build`**: Rejected — the user explicitly values projects with no mistakes; verifying the build is part of "finishing."

### Open Questions (if any)

None — sufficient context exists to write the complete command.

---

## Step-by-Step Tasks

### Step 1: Write the `/finish` Command File

Create `.claude/commands/finish.md` with the full command definition. This file tells Claude exactly how to execute `/finish` when the user types it.

**Actions:**

- Create the file at `.claude/commands/finish.md`
- Write the complete command instructions as detailed below

**Files affected:**

- `.claude/commands/finish.md`

**Full content for `.claude/commands/finish.md`:**

```markdown
# Finish

Completely implement all files in the current project by reading the oldest plan and writing every file's full final code. This turns stub/template files into a working, finished project.

---

## Instructions

### Phase 1: Find the Plan

1. List all `.md` files in `plans/` and select the **oldest one** (earliest modification date — same logic as `/template`).
2. Read the entire plan file from top to bottom. Do not skim.
3. Confirm the plan has a `### Step-by-Step Tasks` section with embedded code blocks for each file.

---

### Phase 2: Write Every File Completely

For each file referenced in the plan's Step-by-Step Tasks:

1. Read the step carefully. Each step specifies a file path and provides the **complete final content** as a code block.
2. Write the file at the exact path listed, using the exact content from the plan's code block.
3. If the file already exists (as a stub), **overwrite it completely** with the final content.
4. If the file does not exist yet, create it.
5. Do not leave any `// TODO` comments from the stub phase. The plan's code blocks are the final implementation.

Work through every step in the plan that creates or modifies a file. Do not skip any.

**Important:** The file content to write is in the Step-by-Step Tasks section, not the "New Files to Create" table. The table only has paths and one-line descriptions. The actual code is in the steps.

---

### Phase 3: Build and Verify

After all files are written:

1. Run `cargo build` in the project root.
2. If it succeeds, continue to Phase 4.
3. If it fails, read the compiler errors carefully and fix them. Common issues:
   - Missing `use` statements
   - Mismatched types between agents (check `src/messages.rs` types match what agents expect)
   - Missing module declarations in `mod.rs` files
4. Re-run `cargo build` after fixes until it passes.
5. Run `cargo run` to confirm the program executes and prints output.

---

### Phase 4: Commit

Commit all created and modified files:

1. Stage all new and modified files (be specific — list the files, not `git add .`)
2. Commit with message:
   ```
   feat: implement complete multi-agent coding system

   All agents (Coordinator, Planner, Coder, Reviewer, Debugger),
   pipeline, message types, and task lifecycle fully implemented.
   Project builds and runs successfully.
   ```

---

### Phase 5: Update Plan Status

1. Open the plan file that was used.
2. Change `**Status:** Draft` to `**Status:** Implemented`.
3. Append this section at the end of the plan file:

```markdown
---

## Implementation Notes

**Implemented:** <today's date>

### Summary

All files fully implemented via /finish command. Project builds and runs successfully with cargo run.

### Deviations from Plan

<None, or list any>

### Issues Encountered

<None, or list any compiler errors and how they were fixed>
```

---

## Report

After completion, provide:

1. **Plan used:** filename of the plan that was read
2. **Files written:** list every file that was created or overwritten with final code
3. **Build result:** confirm `cargo build` and `cargo run` succeeded
4. **Commit:** confirm files were committed
5. **Next steps:** how to run the project (`cargo run`)
```

---

### Step 2: Update CLAUDE.md

Add `/finish` to the Commands section in `CLAUDE.md`.

**Actions:**

- Read `CLAUDE.md`
- Find the `### /template` section
- Add a new `### /finish` section immediately after it

**Files affected:**

- `CLAUDE.md`

**Content to insert after the `/template` section:**

```markdown
### /finish

**Purpose:** Fully implement all stubbed files from the current project plan, producing a complete working project.

When invoked, Claude will:

1. Find the oldest plan in `plans/`
2. Read the complete file content specifications from the plan's Step-by-Step Tasks
3. Write every file completely (overwriting any stubs from `/template`)
4. Run `cargo build` to verify the project compiles
5. Commit all changes to git

Use this after `/template` has scaffolded the files and you're ready to go from stubs to a fully working project.
```

---

### Step 3: Verify the Command File

Confirm the command is correctly created and registered.

**Actions:**

- Read `.claude/commands/finish.md` back to confirm it was written correctly
- Confirm the file is in the right location (`.claude/commands/`)
- Confirm `CLAUDE.md` was updated correctly

**Files affected:**

- None (verification only)

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — will be updated to list `/finish` in Commands section
- `.claude/commands/` — all commands live here; `/finish` follows the same pattern

### Updates Needed for Consistency

- `CLAUDE.md` Commands section needs `/finish` added
- No other files reference the commands directory directly

### Impact on Existing Workflows

- `/prime` will now load context that includes `/finish` as an available command
- The session workflow in `CLAUDE.md` can optionally list `/finish` as an alternative to `/implement`
- No existing commands are changed or removed

---

## Validation Checklist

- [ ] `.claude/commands/finish.md` exists and contains complete instructions
- [ ] `CLAUDE.md` Commands section lists `/finish` with purpose description
- [ ] The `/finish` command instructions clearly describe all 5 phases
- [ ] The command auto-detects the oldest plan (no argument required)
- [ ] The command instructions specify writing complete final code, not stubs
- [ ] The command instructions include running `cargo build` for verification
- [ ] The command instructions include committing changes
- [ ] The command instructions include updating the plan status to "Implemented"

---

## Success Criteria

The implementation is complete when:

1. Running `/finish` in any Claude Code session finds the oldest plan and writes every file with complete final code (no TODOs remaining)
2. After `/finish` runs, `cargo build` and `cargo run` succeed and all 5 agents print their output
3. `CLAUDE.md` correctly documents `/finish` so future sessions know it exists

---

## Notes

- `/finish` is intentionally opinionated about Rust projects because the current workspace project is Rust. If this workspace is later used for other project types, the Phase 3 build step instructions can be generalized (e.g., check for `Cargo.toml` vs `package.json` vs `Makefile` to decide how to build).
- `/finish` complements `/template` — you run `/template` to get navigable stubs, browse the structure, then run `/finish` to turn it into working code.
- The command does not require the user to supply the plan path — this is intentional for beginner-friendliness.

---

## Implementation Notes

**Implemented:** 2026-02-17

### Summary

Created `.claude/commands/finish.md` with the full 5-phase command definition. Updated `CLAUDE.md` to document `/finish` in the Commands section, the workspace structure tree, and the Session Workflow.

### Deviations from Plan

Also updated the CLAUDE.md workspace structure tree (`.claude/commands/` listing) and the Session Workflow section — these were logical consistency updates not explicitly listed in the plan steps but required by the CLAUDE.md maintenance policy.

### Issues Encountered

None.
