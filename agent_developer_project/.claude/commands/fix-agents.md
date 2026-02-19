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
