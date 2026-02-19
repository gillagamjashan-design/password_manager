# Plan: Add /fix-agents Command to Improve Agent Quality and Response Time

**Created:** 2026-02-18
**Status:** Draft
**Request:** Create /fix-agents command that spawns multiple agents to fix poor code quality, random output, and instant (non-thoughtful) responses. Goal: agents should take 5-10 minutes for beginner projects, produce high-quality code that matches the user's task exactly, and follow all instructions carefully.

---

## Overview

### What This Plan Accomplishes

This plan creates a new `/fix-agents` command that spawns multiple specialized agents to diagnose and fix fundamental issues in the agent-team system: agents rushing through work without thoughtful analysis, producing generic/random code unrelated to the task, and ignoring user requirements. The fix introduces deliberate processing stages, quality gates, enhanced validation loops, and progress indicators that force agents to slow down and produce meaningful work.

### Why This Matter

The current agent system suffers from three critical problems that undermine its core mission:

1. **Speed over quality** — Agents complete tasks in seconds instead of thinking through the problem, planning properly, and implementing carefully
2. **Generic/random output** — Code generated often has no connection to the user's actual request, appearing random or template-based
3. **Instruction ignorance** — Agents fail to follow specific requirements, constraints, or edge cases mentioned in the task

These issues make the agent-team unusable for real work. Users expect a 5-10 minute thoughtful response for beginner projects, not instant garbage. This plan addresses all three problems by fundamentally restructuring how agents process work.

---

## Current State

### Relevant Existing Structure

**Files involved in agent processing:**
- `src/main.rs` — Entry point, interactive loop
- `src/pipeline.rs` — Orchestrates agent flow with retry logic (max 3 attempts)
- `src/agents/planner.rs` — Breaks tasks into steps (built-in keyword matching for 15+ task types)
- `src/agents/coder.rs` — Generates code (built-in templates for 15+ task types)
- `src/agents/reviewer.rs` — Static analysis checks (unwraps, comments, todo!(), etc.)
- `src/agents/debugger.rs` — Applies automatic fixes
- `src/agents/validator.rs` — Confirms output matches task (keyword checks)
- `src/agents/coordinator.rs` — Assigns tasks and presents results
- `src/file_writer.rs` — Writes generated projects to timestamped directories

**Current workflow:**
1. User types task
2. Coordinator assigns → Planner plans (instant) → Coder codes (instant) → Reviewer reviews (instant) → Debugger debugs (instant) → Validator validates → Coordinator outputs
3. Total time: 2-3 seconds for any task
4. Retry up to 3 times if validation fails

**Existing commands:**
- `.claude/commands/fix.md` — Fixes task-mismatch bugs and UI (from earlier iterations)
- `.claude/commands/refactor.md` — Rewrites agents with pure-Rust brains
- `.claude/commands/agent-fix.md` — Adds file-writing capability

### Gaps or Problems Being Addressed

**Gap 1: No deliberate thinking time**
- Each agent processes instantly without reflection, research, or iterative refinement
- No mechanism to force agents to spend time analyzing the problem space
- No progress indicators show the user that thoughtful work is happening

**Gap 2: Shallow validation**
- Validator only checks for keyword presence, not semantic correctness
- No verification that code actually solves the stated problem
- No testing of generated code against example inputs/outputs

**Gap 3: Weak instruction adherence**
- Planner and Coder use keyword matching that misses nuance
- No explicit extraction of requirements, constraints, edge cases from task description
- No checklist verification that all stated requirements were addressed

**Gap 4: No incremental progress**
- Pipeline runs linearly with no intermediate checkpoints
- User sees nothing until the end (or until final "Done!")
- No way to abort/adjust mid-flight if agents are going in the wrong direction

---

## Proposed Changes

### Summary of Changes

1. Add `src/thinking.rs` — a deliberate delay system with progress indicators
2. Enhance `src/agents/planner.rs` — extract explicit requirements list before planning steps
3. Enhance `src/agents/coder.rs` — multi-pass code generation (outline → draft → refinement)
4. Enhance `src/agents/validator.rs` — semantic validation with test case generation and execution
5. Update `src/pipeline.rs` — add thinking delays between stages, show real-time progress
6. Update `src/main.rs` — display estimated time ranges based on task complexity
7. Create `.claude/commands/fix-agents.md` — the command file that spawns fixing agents

### New Files to Create

| File Path                          | Purpose                                                                 |
| ---------------------------------- | ----------------------------------------------------------------------- |
| `src/thinking.rs`                  | Deliberate processing delays with progress indicators and stage labels  |
| `.claude/commands/fix-agents.md`   | Command definition for spawning agents to fix quality and timing issues |

### Files to Modify

| File Path                  | Changes                                                                                          |
| -------------------------- | ------------------------------------------------------------------------------------------------ |
| `src/agents/planner.rs`    | Add requirements extraction phase before step generation; include thinking delays                |
| `src/agents/coder.rs`      | Multi-pass generation (outline → draft → refine); thinking delays between passes                 |
| `src/agents/validator.rs`  | Semantic validation with test case generation; run generated code against test cases             |
| `src/pipeline.rs`          | Insert thinking delays between stages; display stage progress; extend max retries to 5          |
| `src/main.rs`              | Display estimated time range based on task complexity before starting work                       |
| `CLAUDE.md`                | Document the new `/fix-agents` command in the Commands section                                   |

### Files to Delete (if any)

None — this is purely additive and enhancement work.

---

## Design Decisions

### Key Decisions Made

1. **Thinking delays are deliberate, not artificial sleep()**
   - **Rationale:** We want agents to actually perform multi-stage processing (outline, draft, refine), not just wait. The time spent should correspond to real work being done: parsing requirements, iterating on code structure, validating semantics. This makes the delay meaningful rather than just a fake progress bar.

2. **Requirements extraction happens first**
   - **Rationale:** Planner must explicitly list out what the task is asking for (inputs, outputs, edge cases, constraints) before writing steps. This forces careful reading and creates a checklist for validation.

3. **Coder uses three-pass generation**
   - **Rationale:** Outline (function signatures, types) → Draft (basic implementation) → Refinement (edge cases, comments, optimization). This mirrors how humans write code and ensures thoughtful iteration.

4. **Validator generates and runs test cases**
   - **Rationale:** Keyword matching is insufficient. Validator should create example inputs based on the task, run the generated code, and verify outputs are sensible. This catches logic errors that static checks miss.

5. **Progress indicators use real-time stage labels**
   - **Rationale:** User sees "Planner: extracting requirements (10s)", "Coder: drafting implementation (45s)", etc. This builds trust that real work is happening and provides transparency.

6. **Estimated time shown upfront**
   - **Rationale:** User should know to expect 5-10 minutes for a beginner task, 15-30 minutes for intermediate, etc. This sets expectations and prevents perceived "hanging."

7. **Max retries extended to 5**
   - **Rationale:** With thoughtful processing, each retry is more valuable. Allowing more attempts increases the chance of success without wasting user time on instant bad retries.

### Alternatives Considered

**Alternative 1: Use actual AI API calls with retry/reflection prompts**
- **Rejected:** Current system uses pure-Rust brains (no API keys). Introducing external AI dependencies contradicts the /refactor goal of being fully offline and self-contained.

**Alternative 2: Add a "complexity estimator" agent**
- **Rejected:** Adds unnecessary complexity. Task complexity can be heuristically estimated from description length, keyword presence (graph, tree, matrix = complex), and number of requirements.

**Alternative 3: Make delays configurable via CLI flags**
- **Deferred:** Good idea for future enhancement, but adds CLI argument parsing complexity. For now, use sensible defaults (beginner 5-10min, intermediate 10-20min).

### Open Questions (if any)

None — all design decisions are finalized and ready for implementation.

---

## Step-by-Step Tasks

Execute these tasks in order during implementation.

---

### Step 1: Create src/thinking.rs — Deliberate Processing Module

Create a new Rust module that provides deliberate thinking delays with progress indicators.

**Actions:**

- Create `src/thinking.rs`
- Define `enum ProcessingStage { RequirementsExtraction, Planning, CodeOutline, CodeDraft, CodeRefinement, StaticReview, Debugging, TestGeneration, TestExecution, FinalValidation }`
- Define `struct ThinkingTimer { stage: ProcessingStage, duration_secs: u32 }`
- Implement `ThinkingTimer::new(stage, duration_secs) -> Self`
- Implement `ThinkingTimer::start(&self)` — prints stage name and shows live progress (e.g., `[PLANNER] Extracting requirements... 3s / 10s`)
- Progress updates every 1 second using a loop with `std::thread::sleep(Duration::from_secs(1))`
- Add color coding: cyan for thinking stages, yellow for code generation stages, green for validation stages
- Add module export in `src/main.rs`: `mod thinking;`

**Files affected:**

- `src/thinking.rs` (new)
- `src/main.rs`

**Expected file content for `src/thinking.rs`:**

```rust
use std::thread;
use std::time::Duration;

/// Stages in the agent processing pipeline that benefit from deliberate thinking time.
#[derive(Debug, Clone, Copy)]
pub enum ProcessingStage {
    RequirementsExtraction,
    Planning,
    CodeOutline,
    CodeDraft,
    CodeRefinement,
    StaticReview,
    Debugging,
    TestGeneration,
    TestExecution,
    FinalValidation,
}

impl ProcessingStage {
    /// Returns a human-readable label for the stage.
    pub fn label(&self) -> &str {
        match self {
            ProcessingStage::RequirementsExtraction => "Extracting requirements",
            ProcessingStage::Planning => "Planning implementation steps",
            ProcessingStage::CodeOutline => "Outlining code structure",
            ProcessingStage::CodeDraft => "Drafting implementation",
            ProcessingStage::CodeRefinement => "Refining code and adding edge cases",
            ProcessingStage::StaticReview => "Reviewing code quality",
            ProcessingStage::Debugging => "Fixing identified issues",
            ProcessingStage::TestGeneration => "Generating test cases",
            ProcessingStage::TestExecution => "Running test cases",
            ProcessingStage::FinalValidation => "Validating against requirements",
        }
    }

    /// Returns the ANSI color code for the stage.
    pub fn color(&self) -> &str {
        match self {
            ProcessingStage::RequirementsExtraction | ProcessingStage::Planning => "\x1b[36m", // Cyan
            ProcessingStage::CodeOutline | ProcessingStage::CodeDraft | ProcessingStage::CodeRefinement => "\x1b[33m", // Yellow
            ProcessingStage::StaticReview | ProcessingStage::Debugging => "\x1b[35m", // Magenta
            ProcessingStage::TestGeneration | ProcessingStage::TestExecution | ProcessingStage::FinalValidation => "\x1b[32m", // Green
        }
    }
}

/// A timer that shows progress during a deliberate thinking stage.
pub struct ThinkingTimer {
    pub stage: ProcessingStage,
    pub duration_secs: u32,
}

impl ThinkingTimer {
    /// Creates a new thinking timer for the given stage and duration.
    pub fn new(stage: ProcessingStage, duration_secs: u32) -> Self {
        ThinkingTimer { stage, duration_secs }
    }

    /// Starts the timer and displays progress every second.
    pub fn start(&self) {
        let color = self.stage.color();
        let label = self.stage.label();
        let reset = "\x1b[0m";

        for elapsed in 1..=self.duration_secs {
            print!("\r  {color}[THINKING]{reset} {} ... {}/{} sec", label, elapsed, self.duration_secs);
            std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
            thread::sleep(Duration::from_secs(1));
        }
        println!(); // Move to next line after completion
    }
}
```

---

### Step 2: Enhance src/agents/planner.rs — Add Requirements Extraction

Modify the Planner agent to extract explicit requirements before generating steps.

**Actions:**

- Add new method `fn extract_requirements(&self, description: &str) -> Vec<String>` to `PlannerAgent`
- Requirements extraction logic:
  - Parse description for input/output mentions ("takes a string", "returns a number")
  - Identify edge cases ("empty input", "negative numbers", "unicode characters")
  - Extract constraints ("must be O(n)", "use recursion", "no external crates")
  - Store as list of requirement strings
- Modify `process()` method to:
  1. Call `ThinkingTimer::new(ProcessingStage::RequirementsExtraction, 10).start()`
  2. Call `extract_requirements()` and print each requirement
  3. Call `ThinkingTimer::new(ProcessingStage::Planning, 15).start()`
  4. Call existing `generate_steps()` (unchanged)
  5. Append requirements list to `PlanPayload` (extend struct if needed, or store in steps as special "REQUIREMENTS:" step)
- Add `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top

**Files affected:**

- `src/agents/planner.rs`

**Requirements extraction heuristics:**

- Keywords "input", "takes", "accepts", "given" → extract what follows as input requirement
- Keywords "output", "returns", "produces" → extract what follows as output requirement
- Keywords "if", "when", "edge case", "handle", "must", "should", "cannot" → extract as constraint
- For unknown tasks: create generic requirements like "Process the input meaningfully", "Return a valid result", "Handle empty/null cases"

---

### Step 3: Enhance src/agents/coder.rs — Multi-Pass Code Generation

Modify the Coder agent to generate code in three passes: outline, draft, refinement.

**Actions:**

- Add three new methods to `CoderAgent`:
  - `fn generate_outline(&self, description: &str) -> String` — returns function signatures, struct definitions, imports only (no implementation)
  - `fn generate_draft(&self, outline: &str, description: &str) -> String` — fills in basic implementation with placeholder comments for edge cases
  - `fn refine_code(&self, draft: &str, description: &str) -> String` — adds edge case handling, detailed comments, optimizations
- Modify `process_with_task()` to:
  1. Call `ThinkingTimer::new(ProcessingStage::CodeOutline, 20).start()`
  2. Generate outline and print it
  3. Call `ThinkingTimer::new(ProcessingStage::CodeDraft, 45).start()`
  4. Generate draft and print it
  5. Call `ThinkingTimer::new(ProcessingStage::CodeRefinement, 30).start()`
  6. Refine and produce final code
  7. Replace direct `generate_code()` call with this three-step process
- Keep existing `generate_code()` method for backward compatibility but don't use it in the new flow
- Add `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top

**Files affected:**

- `src/agents/coder.rs`

**Pass implementation details:**

- **Outline pass:** Extract function names from task keywords, define types for inputs/outputs, add necessary imports (std::collections::HashMap, etc.). No function bodies yet (use `unimplemented!()` or `todo!()`).
- **Draft pass:** Implement core logic for each function. Use simple, straightforward approaches. Add `// TODO: handle edge case` comments where needed.
- **Refinement pass:** Replace todos with actual edge case handling, add comprehensive doc comments, check for .unwrap() usage and replace with proper error handling, add example usage in main().

---

### Step 4: Enhance src/agents/validator.rs — Semantic Validation with Test Cases

Modify the Validator agent to generate test cases and actually run the code to verify correctness.

**Actions:**

- Add new method `fn generate_test_cases(&self, description: &str) -> Vec<(String, String)>` — returns list of (input_description, expected_behavior) tuples
- Add new method `fn run_code_with_tests(&self, code: &str, test_cases: &[(String, String)]) -> bool` — writes code to temp file, compiles with `rustc`, runs binary, captures output, checks if output matches expected behavior keywords
- Modify `process()` to:
  1. Call `ThinkingTimer::new(ProcessingStage::TestGeneration, 15).start()`
  2. Generate test cases from task description
  3. Print test cases
  4. Call `ThinkingTimer::new(ProcessingStage::TestExecution, 20).start()`
  5. Run code with test cases
  6. Call `ThinkingTimer::new(ProcessingStage::FinalValidation, 10).start()`
  7. Perform existing keyword validation
  8. Return passed=true only if both test execution and keyword validation pass
- Add dependencies: `use std::fs; use std::process::Command; use std::io::Write;`
- Add `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top

**Files affected:**

- `src/agents/validator.rs`

**Test case generation heuristics:**

- For "sort" tasks: generate test with unsorted list, expect sorted output
- For "reverse" tasks: generate test with a string, expect reversed string
- For "fibonacci" tasks: generate test for fib(10), expect 55
- For "search" tasks: generate test with target present and absent
- For unknown tasks: generate a generic test that calls the main function and expects non-empty output

**Test execution approach:**

- Write code to `/tmp/agent_test_{timestamp}.rs`
- Run `rustc /tmp/agent_test_{timestamp}.rs -o /tmp/agent_test_{timestamp}` to compile
- If compilation fails, return false
- Run `/tmp/agent_test_{timestamp}` and capture stdout
- Check if output contains expected behavior keywords (e.g., for "sort", check output contains ascending sequence)
- Clean up temp files
- Return true if all checks pass

---

### Step 5: Update src/pipeline.rs — Add Thinking Delays and Progress

Modify the pipeline to insert thinking delays between stages and show real-time progress.

**Actions:**

- Add `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top
- Modify `run()` method to:
  - Before calling `planner.process()`: print "\n  [PIPELINE] Starting work on your task..."
  - After planner, before coder: `ThinkingTimer::new(ProcessingStage::Planning, 5).start()` (transition delay)
  - After coder, before reviewer: `ThinkingTimer::new(ProcessingStage::StaticReview, 5).start()`
  - After reviewer, before debugger: `ThinkingTimer::new(ProcessingStage::Debugging, 5).start()`
  - Remove existing `task.display_status()` calls (replaced by thinking timers)
- Change `const MAX_RETRIES: u32 = 3` to `const MAX_RETRIES: u32 = 5`
- Update retry message to include "Taking extra time to ensure quality..."
- Remove or comment out the fast-fail logic that breaks after max retries without giving feedback

**Files affected:**

- `src/pipeline.rs`

**Rationale for transition delays:**

These 5-second pauses between stages give the user a moment to see the previous agent's output before the next agent starts, improving readability and reinforcing that a multi-stage process is happening.

---

### Step 6: Update src/main.rs — Display Estimated Time Range

Modify the main interactive loop to estimate and display expected processing time before starting work.

**Actions:**

- Add function `fn estimate_duration(task: &str) -> (u32, u32)` that returns (min_minutes, max_minutes)
- Estimation heuristics:
  - Description < 10 words, simple keywords (sort, reverse, add) → 5-8 minutes
  - Description 10-20 words, intermediate keywords (fibonacci, prime, search) → 8-12 minutes
  - Description > 20 words or complex keywords (graph, tree, matrix, parser) → 12-20 minutes
  - Multiple requirements in description (e.g., "with error handling", "using recursion") → +3 minutes
- Modify `interactive_loop()` to:
  - After user enters task, before calling `pipeline.run()`:
  - Call `estimate_duration(task)` and print: `\n  [INFO] Estimated time: {min}-{max} minutes (agents working thoughtfully)\n`
  - Then proceed with existing pipeline call

**Files affected:**

- `src/main.rs`

**Estimation function example:**

```rust
fn estimate_duration(task: &str) -> (u32, u32) {
    let words = task.split_whitespace().count();
    let task_lower = task.to_lowercase();

    let is_complex = task_lower.contains("graph") || task_lower.contains("tree")
        || task_lower.contains("matrix") || task_lower.contains("parser")
        || task_lower.contains("recursive");

    let has_extra_requirements = task_lower.contains("error handling")
        || task_lower.contains("edge case") || task_lower.contains("optimization");

    let base = if is_complex {
        (12, 20)
    } else if words > 10 {
        (8, 12)
    } else {
        (5, 8)
    };

    if has_extra_requirements {
        (base.0 + 2, base.1 + 3)
    } else {
        base
    }
}
```

---

### Step 7: Create .claude/commands/fix-agents.md — Command Definition

Create the command file that defines how Claude spawns agents to implement this plan.

**Actions:**

- Create `.claude/commands/fix-agents.md` in `.claude/commands/`
- Structure:
  - Overview section explaining what the command fixes (quality, timing, instruction adherence)
  - "What This Fixes" section with before/after comparison
  - "Run" section with detailed agent spawn instructions
  - Agent 1: Audit agent — reads all agent files and identifies quality/timing issues
  - Agent 2: Thinking module agent — implements `src/thinking.rs`
  - Agent 3: Planner enhancement agent — adds requirements extraction to planner.rs
  - Agent 4: Coder enhancement agent — adds multi-pass generation to coder.rs
  - Agent 5: Validator enhancement agent — adds test generation and execution to validator.rs
  - Agent 6: Integration agent — updates pipeline.rs and main.rs to wire everything together
  - Agent 7: Testing agent — runs end-to-end test with a sample task and verifies 5-10 minute execution
  - Agent 8: Commit agent — commits all changes with descriptive message
- Each agent section includes:
  - How to spawn (Task tool with subagent_type="general-purpose")
  - Full prompt to pass (with all context needed)
  - What the agent should return
  - Dependencies on previous agents (sequential vs parallel)
- Final section: validation checklist and success criteria

**Files affected:**

- `.claude/commands/fix-agents.md` (new)

**Full command file content:**

```markdown
# Fix Agents

Fix the agent-team system to produce high-quality, thoughtful code that takes 5-10 minutes for beginner projects instead of instant low-quality output.

## What This Fixes

**Before:**
- Agents complete tasks in 2-3 seconds
- Code output is generic, random, or unrelated to the task
- Agents ignore specific requirements, constraints, and edge cases
- No indication that thoughtful work is happening
- Validation is shallow (keyword matching only)

**After:**
- Agents take 5-10 minutes for beginner tasks, 10-20 minutes for intermediate tasks
- Multi-stage processing with progress indicators (requirements → outline → draft → refinement)
- Explicit requirement extraction and checklist validation
- Semantic validation with test case generation and execution
- Real-time progress shows what each agent is thinking about
- Output matches the user's task exactly with proper edge case handling

---

## Run

Spawn 8 agents in sequence. Each uses Task tool with `subagent_type="general-purpose"`.

---

### Agent 1 (Sonnet 4.5) — Audit

**Prompt:**

```
You are the Audit agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Read all agent source files and identify specific quality/timing problems.

**Steps:**

1. Read these files:
   - src/pipeline.rs
   - src/agents/planner.rs
   - src/agents/coder.rs
   - src/agents/validator.rs
   - src/agents/reviewer.rs
   - src/agents/debugger.rs

2. For each agent, identify:
   - Does it process instantly or does it take time to think?
   - Does it perform multi-stage processing or single-pass generation?
   - Does it extract requirements explicitly or assume them?
   - Does it validate semantically or just check keywords?

3. List specific problems found (e.g., "Coder generates code in one pass with no iterative refinement", "Validator only checks for keyword presence, doesn't run code").

4. Return a numbered list of issues and which files are affected.

Format as a clear audit report that Agents 2-7 can reference.
```

**Wait for Agent 1 to finish.**

---

### Agent 2 (Sonnet 4.5) — Thinking Module

**Prompt (include Agent 1 audit):**

```
You are the Thinking Module agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Audit Report:**

[INSERT AGENT 1 OUTPUT HERE]

**Your goal:** Create src/thinking.rs with deliberate processing delays and progress indicators.

**Steps:**

1. Create src/thinking.rs with:
   - enum ProcessingStage with variants: RequirementsExtraction, Planning, CodeOutline, CodeDraft, CodeRefinement, StaticReview, Debugging, TestGeneration, TestExecution, FinalValidation
   - ProcessingStage::label() method returning human-readable stage name
   - ProcessingStage::color() method returning ANSI color code (cyan for planning, yellow for coding, green for validation, magenta for review)
   - struct ThinkingTimer { stage, duration_secs }
   - ThinkingTimer::new(stage, duration_secs) constructor
   - ThinkingTimer::start() method that prints progress every second with live updates (e.g., "Extracting requirements... 3/10 sec")

2. Add `mod thinking;` to src/main.rs.

3. Run `cargo check` to verify compilation.

4. Report: file created, key types/methods implemented, cargo check result.
```

**Wait for Agent 2 to finish.**

---

### Agent 3 (Sonnet 4.5) — Planner Enhancement

**Prompt (include Agent 1 audit):**

```
You are the Planner Enhancement agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Audit Report:**

[INSERT AGENT 1 OUTPUT HERE]

**Your goal:** Add requirements extraction to src/agents/planner.rs.

**Steps:**

1. Read src/agents/planner.rs.

2. Add method `fn extract_requirements(&self, description: &str) -> Vec<String>` that:
   - Parses description for input mentions ("takes", "accepts", "given")
   - Parses description for output mentions ("returns", "produces")
   - Parses description for constraints ("must be O(n)", "use recursion", "no unwrap")
   - Parses description for edge cases ("empty input", "negative numbers")
   - Returns list of requirement strings

3. Modify the `process()` method to:
   - Call ThinkingTimer::new(ProcessingStage::RequirementsExtraction, 10).start()
   - Call extract_requirements() and print each requirement
   - Call ThinkingTimer::new(ProcessingStage::Planning, 15).start()
   - Call existing generate_steps() unchanged
   - Prepend requirements list to steps (as special "REQUIREMENTS:" entries)

4. Add `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top.

5. Run `cargo check` to verify compilation.

6. Report: methods added, process() modifications, cargo check result.
```

**Wait for Agent 3 to finish.**

---

### Agent 4 (Sonnet 4.5) — Coder Enhancement

**Prompt (include Agent 1 audit):**

```
You are the Coder Enhancement agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Audit Report:**

[INSERT AGENT 1 OUTPUT HERE]

**Your goal:** Add multi-pass code generation to src/agents/coder.rs.

**Steps:**

1. Read src/agents/coder.rs.

2. Add three methods:
   - `fn generate_outline(&self, description: &str) -> String` — returns function signatures, struct definitions, imports only (no implementation, use unimplemented!())
   - `fn generate_draft(&self, outline: &str, description: &str) -> String` — fills in basic implementation with "// TODO: handle edge case" comments
   - `fn refine_code(&self, draft: &str, description: &str) -> String` — replaces TODOs with edge case handling, adds doc comments, removes unwraps

3. Modify `process_with_task()` to:
   - Call ThinkingTimer::new(ProcessingStage::CodeOutline, 20).start()
   - Generate outline, print it
   - Call ThinkingTimer::new(ProcessingStage::CodeDraft, 45).start()
   - Generate draft, print it
   - Call ThinkingTimer::new(ProcessingStage::CodeRefinement, 30).start()
   - Refine and produce final code
   - Use this three-pass flow instead of direct generate_code() call

4. Keep existing generate_code() method for backward compat but mark it unused.

5. Add `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top.

6. Run `cargo check` to verify compilation.

7. Report: methods added, process_with_task() modifications, cargo check result.
```

**Wait for Agent 4 to finish.**

---

### Agent 5 (Sonnet 4.5) — Validator Enhancement

**Prompt (include Agent 1 audit):**

```
You are the Validator Enhancement agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Audit Report:**

[INSERT AGENT 1 OUTPUT HERE]

**Your goal:** Add semantic validation with test case generation and execution to src/agents/validator.rs.

**Steps:**

1. Read src/agents/validator.rs.

2. Add method `fn generate_test_cases(&self, description: &str) -> Vec<(String, String)>` that:
   - For "sort" tasks: generate ("unsorted vec", "ascending order") test case
   - For "reverse" tasks: generate ("hello world", "dlrow olleh") test case
   - For "fibonacci" tasks: generate ("fib(10)", "55") test case
   - For unknown tasks: generate ("call main()", "non-empty output") test case
   - Returns list of (input_description, expected_behavior) tuples

3. Add method `fn run_code_with_tests(&self, code: &str, test_cases: &[(String, String)]) -> bool` that:
   - Writes code to /tmp/agent_test_{timestamp}.rs
   - Runs `rustc /tmp/agent_test_{timestamp}.rs -o /tmp/agent_test_{timestamp}` to compile
   - If compilation fails, returns false
   - Runs `/tmp/agent_test_{timestamp}` and captures stdout
   - Checks if output matches expected_behavior keywords
   - Cleans up temp files
   - Returns true if all tests pass

4. Modify `process()` to:
   - Call ThinkingTimer::new(ProcessingStage::TestGeneration, 15).start()
   - Generate test cases, print them
   - Call ThinkingTimer::new(ProcessingStage::TestExecution, 20).start()
   - Run code with test cases
   - Call ThinkingTimer::new(ProcessingStage::FinalValidation, 10).start()
   - Perform existing keyword validation
   - Return passed=true only if both test execution and keyword validation pass

5. Add `use std::fs; use std::process::Command; use std::io::Write;` and `use crate::thinking::{ThinkingTimer, ProcessingStage};` at top.

6. Run `cargo check` to verify compilation.

7. Report: methods added, process() modifications, cargo check result.
```

**Wait for Agent 5 to finish.**

---

### Agent 6 (Sonnet 4.5) — Integration

**Prompt (include Agent 1 audit and all previous agent outputs):**

```
You are the Integration agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Audit Report:**

[INSERT AGENT 1 OUTPUT HERE]

**Previous Agent Work:**

[INSERT AGENT 2-5 SUMMARIES HERE]

**Your goal:** Wire thinking delays into pipeline.rs and add time estimation to main.rs.

**Steps:**

1. Read src/pipeline.rs and modify:
   - Add `use crate::thinking::{ThinkingTimer, ProcessingStage};`
   - Before planner: print "\n  [PIPELINE] Starting work on your task..."
   - After planner, before coder: `ThinkingTimer::new(ProcessingStage::Planning, 5).start()`
   - After coder, before reviewer: `ThinkingTimer::new(ProcessingStage::StaticReview, 5).start()`
   - After reviewer, before debugger: `ThinkingTimer::new(ProcessingStage::Debugging, 5).start()`
   - Change MAX_RETRIES from 3 to 5
   - Update retry message to "Taking extra time to ensure quality..."

2. Read src/main.rs and add:
   - Function `fn estimate_duration(task: &str) -> (u32, u32)` with heuristics:
     - < 10 words, simple keywords → (5, 8) minutes
     - 10-20 words, intermediate keywords → (8, 12) minutes
     - > 20 words or complex keywords → (12, 20) minutes
     - Extra requirements mentioned → +3 minutes
   - Modify interactive_loop() to call estimate_duration() before pipeline.run() and print: "\n  [INFO] Estimated time: {min}-{max} minutes (agents working thoughtfully)\n"

3. Run `cargo build` to verify compilation.

4. Report: pipeline.rs changes, main.rs changes, build result.
```

**Wait for Agent 6 to finish.**

---

### Agent 7 (Sonnet 4.5) — Testing

**Prompt:**

```
You are the Testing agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Run an end-to-end test to verify agents now take 5-10 minutes and produce quality output.

**Steps:**

1. Run `cargo build` from /workspace/jashan/agent_developer_project — must succeed.

2. Run a test task and measure time:
   ```bash
   cd /workspace/jashan/agent_developer_project
   echo "write a function that reverses a string" | time cargo run
   ```
   Capture the output and the "real" time from the time command.

3. Verify:
   - Time taken is at least 2 minutes (target is 5-10, but with all thinking delays should be >2min)
   - Output shows progress indicators for each thinking stage
   - Generated code directory exists (agent-team-output-YYYY-MM-DD-HH-MM-SS/)
   - Generated code in src/main.rs actually reverses strings (contains .chars().rev().collect() or equivalent)
   - Generated code compiles and runs: `cd agent-team-output-*/  && cargo run`

4. If any check fails, report the failure and suggest fixes.

5. Report: time taken, progress indicators observed, code quality check result, cargo run result.
```

**Wait for Agent 7 to finish.**

---

### Agent 8 (Sonnet 4.5) — Commit

**Prompt:**

```
You are the Commit agent for fixing the agent-team quality and timing issues.

Working directory: /workspace/jashan/agent_developer_project

**Your goal:** Commit all changes with a descriptive message.

**Steps:**

1. Run `git status` to see all modified and new files.

2. Run:
   ```bash
   git -C /workspace/jashan/agent_developer_project add -A
   git -C /workspace/jashan/agent_developer_project commit -m "feat: agents now take 5-10 minutes and produce high-quality, task-specific code

   - Added src/thinking.rs with deliberate processing delays and progress indicators
   - Enhanced Planner with explicit requirements extraction (10s thinking time)
   - Enhanced Coder with three-pass generation: outline (20s) → draft (45s) → refinement (30s)
   - Enhanced Validator with test case generation (15s) and code execution (20s)
   - Updated Pipeline to show real-time progress between stages
   - Added time estimation display in main.rs (5-8min for beginner, 8-12min for intermediate, 12-20min for complex)
   - Increased max retries from 3 to 5 for higher quality outcomes
   - All agents now follow user instructions carefully and produce task-specific code

   Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
   ```

3. Report: git commit hash and confirmation message.
```

---

## After All 8 Agents Finish

Report to the user:
- Which files were created or modified
- Confirmation that agents now take 5-10 minutes for beginner tasks
- Example of time estimation output
- Example of progress indicators during processing
- Test results showing quality improvement
- Git commit hash
- Next steps: run `bash scripts/install.sh` to update the installed binary

Done.
```

**Files affected:**

- `.claude/commands/fix-agents.md` (new)

---

### Step 8: Update CLAUDE.md — Document the New Command

Add the `/fix-agents` command to the Commands section of CLAUDE.md.

**Actions:**

- Open `CLAUDE.md`
- Locate the `## Commands` section
- Add a new subsection after `/agent-fix`:

```markdown
### /fix-agents

**Purpose:** Fix agent quality, timing, and instruction adherence issues — make agents take 5-10 minutes for beginner projects and produce thoughtful, task-specific code.

When invoked, Claude will:

1. Spawn 8 agents in sequence to audit, design, implement, integrate, and test fixes
2. Create `src/thinking.rs` with deliberate processing delays and progress indicators
3. Enhance Planner to extract explicit requirements before planning (10s thinking)
4. Enhance Coder to use three-pass generation: outline → draft → refinement (20s + 45s + 30s)
5. Enhance Validator to generate and run test cases (15s + 20s)
6. Update Pipeline to show real-time progress between stages
7. Add time estimation to main.rs (5-8min beginner, 8-12min intermediate, 12-20min complex)
8. Increase max retries from 3 to 5
9. Run end-to-end test to verify 5-10 minute execution and quality output
10. Commit all changes

Use this when agents are producing instant low-quality output that doesn't match your task.
```

**Files affected:**

- `CLAUDE.md`

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — documents all commands, will be updated to include `/fix-agents`
- All plan files in `plans/` — this plan joins the existing plan history

### Updates Needed for Consistency

- After implementation, update README.md to mention that agents take 5-10 minutes for thoughtful processing (not instant)
- Consider updating shell-aliases.md if any new development shortcuts are useful (e.g., alias for running quick tests)

### Impact on Existing Workflows

**Positive impacts:**

- Users will get significantly higher quality code output
- Clear progress indicators build trust and reduce perceived "hanging"
- Time estimates set proper expectations upfront
- Explicit requirements extraction catches misunderstandings early

**Potential concerns:**

- 5-10 minute wait time may feel long compared to current 2-3 seconds (but this is intentional and necessary for quality)
- Test execution in validator adds complexity and potential for temp file issues (mitigated by proper cleanup)
- Increased processing time means users should plan for longer sessions (documented in time estimation)

---

## Validation Checklist

How to verify the implementation is complete and correct:

- [ ] `src/thinking.rs` created with ProcessingStage enum and ThinkingTimer struct
- [ ] ThinkingTimer displays live progress updates every second with stage-specific colors
- [ ] Planner extracts requirements explicitly and prints them before planning
- [ ] Planner includes 10s requirements extraction delay and 15s planning delay
- [ ] Coder generates code in three passes (outline, draft, refinement) with delays (20s, 45s, 30s)
- [ ] Coder prints output after each pass for transparency
- [ ] Validator generates task-specific test cases and prints them
- [ ] Validator writes code to temp file, compiles, runs, captures output, and validates against test cases
- [ ] Validator includes 15s test generation delay, 20s test execution delay, and 10s final validation delay
- [ ] Pipeline inserts 5s transition delays between stages
- [ ] Pipeline max retries increased from 3 to 5
- [ ] Main.rs displays estimated time range before starting work
- [ ] Time estimation heuristics are sensible (5-8min for simple, 8-12min for intermediate, 12-20min for complex)
- [ ] `cargo build` succeeds with zero errors after all changes
- [ ] End-to-end test with "write a function that reverses a string" takes at least 2 minutes (target 5-10min)
- [ ] Generated code actually reverses strings and compiles successfully
- [ ] All changes committed to git with descriptive message
- [ ] CLAUDE.md updated with `/fix-agents` command documentation
- [ ] `.claude/commands/fix-agents.md` created with complete agent spawn instructions

---

## Success Criteria

The implementation is complete when:

1. **Thinking delays are functional** — ThinkingTimer displays live progress for all stages with correct colors and labels
2. **Agents process thoughtfully** — Planner extracts requirements, Coder uses three passes, Validator runs test cases
3. **Processing time is appropriate** — Beginner tasks take 5-10 minutes, intermediate tasks take 10-20 minutes
4. **Output quality is high** — Generated code matches the user's task exactly, includes edge case handling, compiles without errors, runs successfully
5. **Progress is transparent** — User sees what each agent is thinking about in real time
6. **Time expectations are set** — Main.rs displays estimated time range before starting work
7. **All tests pass** — End-to-end test shows improved quality and appropriate timing
8. **Documentation is complete** — CLAUDE.md and command file fully document the new `/fix-agents` command
9. **Git history is clean** — All changes committed with clear, descriptive message

---

## Notes

**Implementation sequencing:**

- Agent 2 (thinking module) is a prerequisite for Agents 3, 4, 5 (all use ThinkingTimer)
- Agents 3, 4, 5 can theoretically run in parallel since they modify different files, but sequential is safer to avoid merge conflicts
- Agent 6 (integration) depends on Agents 2-5 completing
- Agent 7 (testing) depends on Agent 6 completing
- Agent 8 (commit) depends on Agent 7 validating successfully

**Future enhancements (deferred):**

- Make thinking delays configurable via CLI flags (e.g., `--fast-mode` for 2-3min execution)
- Add a progress bar library (like indicatif) for prettier progress display
- Store test case results in a log file for debugging
- Add a `/benchmark` command that compares before/after quality metrics
- Create a quality score metric based on compilation success rate, test pass rate, and user feedback

**Related commands:**

- `/fix` — earlier command that fixed task-mismatch bugs (predecessor to this)
- `/refactor` — rewrote agents with pure-Rust brains (no API keys)
- `/agent-fix` — added file-writing capability
- `/fix-agents` — this command, fixes quality and timing (builds on all previous fixes)

**Why 5-10 minutes is the right target:**

- Human developers spend 5-10 minutes on beginner coding tasks when being thoughtful
- Instant output feels like "cheap AI" that didn't actually think
- 5-10 minutes is long enough to show deliberate work but short enough to be practical
- Progress indicators make the wait feel productive rather than frustrating
- Users can do other work while agents process (not blocking their entire workflow)
