# Plan: Add /project-testing Command

**Created:** 2026-02-17
**Status:** Implemented
**Request:** Create a `/project-testing` command that reads the oldest plan to understand how the agents are supposed to behave, runs the project, checks whether the actual runtime behavior matches the plan's expectations, calls an agent to fix anything wrong, and only reports "done" when everything works as the plan describes.

---

## Overview

### What This Plan Accomplishes

This plan creates a new slash command `/project-testing` in `.claude/commands/project-testing.md`. When run, Claude reads the oldest plan in `plans/` to understand how each agent is supposed to behave — what they should do, what output they should print, how they should hand off to each other. It then runs the project with `cargo run`, captures the actual output, compares it to the plan's behavioral expectations, and dispatches a fix agent if anything doesn't match. The loop continues until the running project behaves exactly as the plan describes.

### Why This Matters

After a project is built, there's no way to confirm it actually works the way the plan intended. The plan describes what each agent does at runtime — the `/project-testing` command makes sure the real program actually does those things. It gives the project a self-healing integration test that uses the plan as its test specification.

---

## Current State

### Relevant Existing Structure

```
.claude/commands/
├── prime.md          # Session init
├── create-plan.md    # Plan creation
├── template.md       # Scaffold stubs from plan
├── implement.md      # Execute plan step-by-step
└── finish.md         # Fully implement + build + commit

plans/
├── 2026-02-17-rust-multi-agent-coding-system.md   (Status: Implemented)
├── 2026-02-17-add-template-command.md
└── 2026-02-17-add-finish-command.md

src/
├── main.rs
├── messages.rs
├── task.rs
├── pipeline.rs
└── agents/
    ├── mod.rs
    ├── coordinator.rs
    ├── planner.rs
    ├── coder.rs
    ├── reviewer.rs
    └── debugger.rs
```

The Rust multi-agent plan (`2026-02-17-rust-multi-agent-coding-system.md`) describes expected agent behaviors in detail — for example: agents print labeled output (`[COORDINATOR]`, `[PLANNER]`, `[CODER]`, `[REVIEWER]`, `[DEBUGGER]`), they run in a sequential pipeline, they produce a final code output, and the coordinator starts and ends the session.

### Gaps or Problems Being Addressed

- No command exists to run the project and verify its runtime behavior against plan expectations
- After `/finish`, there is no check that the program actually runs and behaves the way the plan describes
- If an agent is silent when it should print, produces wrong output, or the pipeline doesn't complete, nothing catches it

---

## Proposed Changes

### Summary of Changes

- Create `.claude/commands/project-testing.md` — the new `/project-testing` command
- Update `CLAUDE.md` — add `/project-testing` to the Commands section and Session Workflow

### New Files to Create

| File Path | Purpose |
| --------- | ------- |
| `.claude/commands/project-testing.md` | The `/project-testing` command that reads the oldest plan for behavioral expectations, runs the project, checks the output, dispatches fix agents, and loops until passing |

### Files to Modify

| File Path | Changes |
| --------- | ------- |
| `CLAUDE.md` | Add `/project-testing` to the Commands section and Session Workflow |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **Read the oldest plan for behavioral expectations, not source files**: The plan describes what each agent should do at runtime. That is the test specification. Source files are not read — the running output is what gets checked.

2. **`cargo run` is the test runner**: Running the project and capturing its stdout is how we know if it works. The plan's behavioral descriptions (what agents print, what they do, what order they run in) are compared against this output.

3. **Extract behavioral expectations from the plan's Design Decisions and Step-by-Step Tasks sections**: These sections describe what each agent does at runtime — e.g., "Each agent prints its reasoning steps with clear labels", "Coordinator starts and ends the session", "Pipeline runs Plan → Code → Review → Debug → Output". These become the test cases.

4. **Use the Task tool with general-purpose agent for fixes**: When behavior doesn't match, Claude dispatches a focused fix agent with a precise description of what the running output showed vs. what the plan expects. The agent fixes the relevant source file(s) and returns.

5. **Max 5 fix rounds**: Prevents infinite loops. After 5 rounds without passing, stops and reports what still doesn't match so the user can intervene.

6. **Re-run the project after every fix**: After a fix agent returns, always re-run `cargo run` and re-check all behavioral expectations fresh — don't assume the fix worked.

### Alternatives Considered

- **Check source files for structs/functions instead of running the project**: Rejected — static analysis misses runtime behavior. The plan describes what agents DO, not just what code exists. Running the project is the only honest test.
- **Write actual Rust test cases**: Rejected — requires modifying the Rust project and adds complexity. The command should work without touching the project's source structure.
- **Read the newest plan instead of oldest**: Would break consistency with `/template` and `/finish`. Rejected.

### Open Questions (if any)

None.

---

## Step-by-Step Tasks

### Step 1: Create `.claude/commands/project-testing.md`

Create the command file with the full instructions Claude will follow when `/project-testing` is invoked.

**Actions:**

- Create the file at `.claude/commands/project-testing.md`
- Write the complete command content as specified below

**Files affected:**

- `.claude/commands/project-testing.md`

**Full file content:**

```markdown
# Project Testing

Read the oldest plan to understand how the project is supposed to behave at runtime, then run the project and check that it actually behaves that way. If anything is wrong, call an agent to fix it. Only report "done" when every behavioral expectation from the plan is met.

---

## Instructions

### Phase 1: Find the Plan and Extract Behavioral Expectations

1. List all `.md` files in `plans/` and select the **oldest one** (earliest modification date — same as `/template` and `/finish`).
2. Read the entire plan file from top to bottom. Do not skim.
3. Focus on these sections to build a list of **Behavioral Expectations** — things the running program must do:
   - **"### Design Decisions"** — describes how agents behave (e.g., "each agent prints labeled output", "pipeline runs sequentially", "coordinator starts and ends")
   - **"### Step-by-Step Tasks"** — describes what each agent does at runtime (what it prints, what it produces, what order things happen in)
   - Any description of expected terminal output, agent behavior, or program flow
4. Write out a numbered **Behavioral Expectations List**. Each item is one thing the running program must do. Examples of how to phrase them:
   - "Program runs to completion without panicking"
   - "Output contains [COORDINATOR] label"
   - "Output contains [PLANNER] label"
   - "Output contains [CODER] label"
   - "Output contains [REVIEWER] label"
   - "Output contains [DEBUGGER] label"
   - "Agents run in sequential order (COORDINATOR → PLANNER → CODER → REVIEWER → DEBUGGER)"
   - "A final code result is printed"
   - "Program exits with code 0"
   - (Add any others stated in the plan)

   Base these entirely on what the plan actually says — don't invent expectations that aren't in the plan.

---

### Phase 2: Build the Project

1. Run `cargo build` in the project root.
2. If it **fails**: the project cannot be tested. Report the compiler errors and stop. Tell the user to run `/finish` or `/implement` to fix the build before running `/project-testing` again.
3. If it **succeeds**: continue to Phase 3.

---

### Phase 3: Run the Project and Capture Output

1. Run `cargo run` in the project root.
2. Capture the full stdout and stderr output.
3. Note whether the program:
   - Completed without panicking
   - Exited with code 0
   - Produced any output at all

---

### Phase 4: Check Each Behavioral Expectation

Go through the Behavioral Expectations List one item at a time. For each:

- Read the captured output.
- Determine if the expectation is met (PASS) or not (FAIL).
- Note exactly what was found vs. what was expected for any FAIL.

Build a **Results Table**:

| # | Expectation | Status | Notes |
|---|-------------|--------|-------|
| 1 | Program runs to completion without panicking | PASS | — |
| 2 | Output contains [PLANNER] label | FAIL | Label not found in output |
| 3 | Agents run in sequential order | PASS | — |
| … | … | … | … |

---

### Phase 5: Fix Failures (if any)

If all items are PASS, skip to Phase 6.

If any items are FAIL:

1. Check the fix attempt count. If this is the **5th consecutive attempt** with failures remaining, stop and go to Phase 7.
2. For each FAIL item, write a precise fix description that includes:
   - What the expectation was (from the plan)
   - What the actual output showed
   - What needs to change in the program so the output matches the expectation
3. Combine all fix descriptions into one task prompt for the fix agent.
4. Use the **Task tool** with `subagent_type: "general-purpose"` to dispatch the fix agent. The prompt must:
   - State clearly what the program is (a Rust multi-agent system)
   - State each failing expectation and what the actual output showed
   - Instruct the agent to read the relevant source files and fix them so the runtime output matches the plan's expectations
   - Instruct the agent NOT to change what the agents do conceptually — only fix them so their output matches what the plan says they should output
5. After the fix agent returns, go back to **Phase 2** (rebuild and re-run from scratch). Do not skip the build step.

---

### Phase 6: Declare Done

All behavioral expectations are PASS. Report:

```
## Project Testing: DONE

All behavioral expectations from [plan filename] are satisfied.

### Expectations Checked
[paste the full Results Table — all PASS]

### Fix rounds needed: [N]
```

---

### Phase 7: Report Unresolved Failures (max rounds reached)

5 fix rounds have been attempted and failures remain. Report:

```
## Project Testing: STOPPED — Manual Intervention Needed

5 fix rounds attempted. The following expectations still fail:

[list each FAIL item with what was expected vs. what the output showed]

### Suggested next steps
- Review the failing expectations above against the plan
- Run `/implement plans/<plan-filename>.md` to re-execute the relevant plan steps
- Or manually edit the source files to match the plan's described behavior
- Then run `/project-testing` again
```

---

## Notes

- This command reads the plan for expected **behavior**, then runs the project to check **actual behavior**. It does not read or scan source files.
- Always re-run `cargo run` after every fix — never assume a fix worked without re-checking the actual output.
- The oldest plan in `plans/` is always used — same convention as `/template` and `/finish`.
- If the plan has no Design Decisions or Step-by-Step Tasks sections describing runtime behavior, report this and stop — the plan does not have enough information to test against.
```

---

### Step 2: Update `CLAUDE.md`

Add `/project-testing` to the Commands section and update the Session Workflow section.

**Actions:**

- Read `CLAUDE.md`
- After the `/finish` subsection in the Commands section, add the `/project-testing` subsection:

```markdown
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
```

- In the Session Workflow numbered list, change step 5 (currently "**Finish**") to read:

```markdown
5. **Finish**: Use `/finish` to fully implement all files from the plan (build + commit included)
6. **Test**: Use `/project-testing` to confirm the running project matches the plan's behavioral expectations
7. **Execute (alternative)**: Use `/implement` to execute plans step-by-step with a specific plan path
```

(Renumber the existing step 6 to step 7.)

**Files affected:**

- `CLAUDE.md`

---

### Step 3: Commit the changes

**Actions:**

- Stage `.claude/commands/project-testing.md` and `CLAUDE.md`
- Commit with message: `feat: add /project-testing command for behavioral plan verification`

**Files affected:**

- `.claude/commands/project-testing.md`
- `CLAUDE.md`

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — lists all commands and the session workflow; must be updated
- `.claude/commands/finish.md` — the command that runs before `/project-testing` in the workflow

### Updates Needed for Consistency

- `CLAUDE.md` Commands section must list `/project-testing`
- `CLAUDE.md` Session Workflow section must add `/project-testing` as step 6

### Impact on Existing Workflows

`/project-testing` slots in after `/finish`:

```
/create-plan → /template → /finish → /project-testing
```

No existing commands are modified or broken.

---

## Validation Checklist

- [ ] `.claude/commands/project-testing.md` exists
- [ ] Command Phase 1 extracts behavioral expectations from the plan (not from source files)
- [ ] Command Phase 2 runs `cargo build` and stops if it fails
- [ ] Command Phase 3 runs `cargo run` and captures output
- [ ] Command Phase 4 checks each expectation against the actual captured output
- [ ] Command Phase 5 dispatches a fix agent via Task tool when expectations fail
- [ ] Command Phase 5 re-runs from build after every fix (not just re-checks output)
- [ ] Command has a 5-round max and Phase 7 report for unresolved failures
- [ ] Command only declares "done" in Phase 6 when all expectations PASS
- [ ] `CLAUDE.md` Commands section lists `/project-testing`
- [ ] `CLAUDE.md` Session Workflow lists `/project-testing` as step 6
- [ ] Changes committed to git

---

## Success Criteria

The implementation is complete when:

1. `/project-testing` exists as a runnable Claude Code command
2. The command's instructions clearly distinguish between reading the plan for behavioral expectations and running the project to check actual behavior — no source file scanning
3. The fix loop correctly re-builds and re-runs the project after every fix, not just re-checks the old output
4. `CLAUDE.md` is updated to reflect the new command and workflow step

---

## Notes

- The key distinction from a naive implementation: this command does NOT look at source files to check for structs or functions. It reads the plan → runs the program → compares output. The plan is the spec; the running program is the system under test.
- Future enhancement: support passing a specific plan path as an argument (e.g., `/project-testing plans/my-plan.md`)
- Future enhancement: support `cargo test` once the project has a test suite

---

## Implementation Notes

**Implemented:** 2026-02-17

### Summary

Created `.claude/commands/project-testing.md` with the full 7-phase command (extract expectations, build, run, check, fix loop, done, stop). Updated `CLAUDE.md` to document the command in the Commands section, added it to the workspace structure tree, and added it as step 6 in the Session Workflow.

### Deviations from Plan

The commit only includes `CLAUDE.md` — `.claude/commands/project-testing.md` is excluded from git by the parent workspace's `.gitignore` (which ignores `.claude/` directories). The command file was created successfully on disk and is fully functional in Claude Code.

### Issues Encountered

None beyond the git tracking limitation noted above.
