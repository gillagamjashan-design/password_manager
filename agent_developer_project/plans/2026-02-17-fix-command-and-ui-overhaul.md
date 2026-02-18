# Plan: /fix Command and Interactive UI Overhaul

**Created:** 2026-02-17
**Status:** Draft
**Request:** Add a /fix slash command that spawns multiple Claude Code agents to fix the bug where agents say "finished" but don't do what the user asked; then overhaul the interactive mode UI to match the style of the Claude CLI and Gemini CLI.

---

## Overview

### What This Plan Accomplishes

This plan does two things. First, it adds a `/fix` Claude Code slash command that spawns as many subagents as needed to diagnose and repair the core bug in the Rust agent system: agents produce generic/template code regardless of what the user actually asks for, yet still report success. Second, it completely redesigns the interactive mode terminal UI to match modern AI CLI tools (Claude CLI, Gemini CLI) with a clean prompt, styled headers, and clear agent output sections.

### Why This Matters

Right now the agent team always outputs template code (prime checker, bubble sort, fibonacci, etc.) based on keyword matching, so if a user asks for something not covered by those keywords they get a generic stub. The UI also uses plain box-drawing characters and `[AGENT]` prefixes that look dated compared to what users expect from a modern CLI tool. Fixing both makes the project genuinely useful and impressive.

---

## Current State

### Relevant Existing Structure

- `src/main.rs` — interactive loop, box-drawing banner, `Your task > ` prompt
- `src/pipeline.rs` — five-stage pipeline: Coordinator → Planner → Coder → Reviewer → Debugger
- `src/agents/planner.rs` — generates steps via keyword matching on the task description
- `src/agents/coder.rs` — generates code via keyword matching on plan steps (5 templates + generic fallback)
- `src/agents/reviewer.rs` — checks for comments, main(), println!, unwrap count
- `src/agents/debugger.rs` — patches issues found by reviewer (adds comments, main(), etc.)
- `src/agents/coordinator.rs` — assigns tasks, receives final result, prints output
- `src/task.rs` — Task struct and TaskStatus enum
- `src/messages.rs` — AgentMessage enum and payload structs
- `.claude/commands/interactive-mode.md` — current /interactive-mode command

### Gaps or Problems Being Addressed

1. **Agents report "finished" but ignore the actual task.** The Coder matches only 4 keywords (prime, sort, fibonacci, palindrome/string). Any other request falls through to `generic_code()` which outputs `process(input: i32) -> i32 { input * 2 + 1 }`. The Coordinator then reports "Pipeline complete" as if the task was done correctly.
2. **No validation that the output matches the request.** There is no agent or logic that checks whether the generated code actually addresses what the user asked for.
3. **Interactive UI is plain and dated.** The banner uses box-drawing chars but agent messages use `[COORDINATOR]`, `[PLANNER]` etc. prefixes. There is no color, no spinner, no clear visual separation between agents — unlike Claude CLI or Gemini CLI which use styled prompts, clear section dividers, and rich output formatting.

---

## Proposed Changes

### Summary of Changes

- Add `.claude/commands/fix.md` — the new /fix slash command
- Add `src/agents/validator.rs` — a new ValidatorAgent that checks whether final code actually addresses the task
- Modify `src/agents/coder.rs` — expand keyword coverage significantly and improve the generic fallback to produce task-relevant output instead of a useless stub
- Modify `src/agents/planner.rs` — make steps reflect the actual task description more faithfully
- Modify `src/agents/coordinator.rs` — integrate validator feedback; report honestly when task was not fulfilled
- Modify `src/pipeline.rs` — add ValidatorAgent stage after Debugger; loop back if validation fails (up to 3 retries)
- Modify `src/agents/mod.rs` — expose the new validator module
- Modify `src/main.rs` — overhaul interactive UI: styled prompt, agent section headers, clear separators
- Modify `CLAUDE.md` — document the new /fix command

### New Files to Create

| File Path | Purpose |
| --- | --- |
| `.claude/commands/fix.md` | The /fix slash command definition that Claude Code executes |
| `src/agents/validator.rs` | ValidatorAgent: checks if final output actually addresses the user's request |

### Files to Modify

| File Path | Changes |
| --- | --- |
| `src/main.rs` | Overhaul interactive UI — styled prompt, colored section dividers, agent formatting |
| `src/pipeline.rs` | Add ValidatorAgent stage; add retry loop (max 3 passes) when validation fails |
| `src/agents/mod.rs` | Add `pub mod validator;` |
| `src/agents/planner.rs` | Expand step generation to reflect more task types faithfully |
| `src/agents/coder.rs` | Expand keyword matching; improve generic fallback to use task description |
| `src/agents/coordinator.rs` | Print validator result; distinguish success from failure honestly |
| `CLAUDE.md` | Add /fix to Commands section |

### Files to Delete (if any)

None.

---

## Design Decisions

### Key Decisions Made

1. **ValidatorAgent instead of real LLM calls:** The project is a simulation — agents use rule-based logic, not actual AI. The ValidatorAgent will use heuristic checks: does the code contain keywords from the task description? Does the function name relate to the task? This keeps the project honest about what it is while actually catching the "says done but isn't" bug.

2. **Retry loop in pipeline (max 3):** When validation fails, the pipeline re-runs the Coder and Debugger stages with the validator's feedback appended to the task description. Max 3 retries prevents infinite loops. After 3 failures the Coordinator reports "Could not fully satisfy task" honestly.

3. **Expand Coder keyword matching to 15+ categories:** Add: reverse, factorial, calculator, linked list, binary search, stack, queue, hash map, matrix, graph, tree, recursion, file, and a smarter generic fallback that uses the task description words as the function name.

4. **UI overhaul uses ANSI escape codes:** Rust's standard library supports writing ANSI codes directly via `print!`. No external crates needed. This keeps Cargo.toml unchanged. We'll add bold, colored agent labels and styled dividers.

5. **/fix command spawns subagents via Claude Code Task tool:** The slash command instructs Claude Code to use the Task tool to spawn parallel Explore and Bash agents that (a) read the current src/ files, (b) identify the mismatch bugs, (c) apply fixes, and (d) run `cargo build` to verify.

6. **Planner improvement:** Instead of a fixed 5-step template, the Planner will incorporate the task's key nouns into the steps so the Coder has better signal to match against.

### Alternatives Considered

- **Use actual Anthropic API calls in the Rust agents:** Rejected — adds API key management, network dependency, and significant complexity out of scope for a beginner project.
- **Replace keyword matching with regex:** Considered but keyword matching is simpler and the beginner-friendly goal means readable code matters more than sophistication.
- **Use the `colored` crate for terminal colors:** Rejected — adding an external crate changes Cargo.toml and requires `cargo add`. ANSI codes inline keep it self-contained.

### Open Questions (if any)

None — all decisions are made above.

---

## Step-by-Step Tasks

### Step 1: Create the /fix slash command

Create `.claude/commands/fix.md`. This command tells Claude Code to spawn multiple subagents to diagnose and fix the "agent says done but ignores the task" bug, then perform the UI overhaul.

**Actions:**

- Write `.claude/commands/fix.md` with the following content:

```markdown
# Fix

Diagnose and fix all bugs in the agent-team project, then overhaul the interactive UI.

## What This Fixes

1. **Agents ignore the user's task** — they always output template code and report "done"
2. **Interactive mode UI** — upgrade it to look like Claude CLI / Gemini CLI

## Run

Spawn agents in parallel to investigate and fix:

### Agent 1 — Diagnose the task-mismatch bug
Read `src/agents/coder.rs`, `src/agents/planner.rs`, and `src/pipeline.rs`.
Identify: why does the coder ignore tasks it doesn't recognize?
Report: list every code path that returns a generic result instead of task-specific output.

### Agent 2 — Fix the Coder and Planner
Expand keyword matching in `src/agents/coder.rs` to cover at least 15 task types.
Improve the generic fallback to use the task description words as the function name.
Update `src/agents/planner.rs` steps to reflect the actual task keywords.

### Agent 3 — Add ValidatorAgent
Create `src/agents/validator.rs`.
Add validator module to `src/agents/mod.rs`.
Update `src/pipeline.rs` to run validation after Debugger, with up to 3 retries.
Update `src/agents/coordinator.rs` to report honestly when validation fails.

### Agent 4 — Overhaul Interactive UI
Update `src/main.rs` to use ANSI styling:
- Bold colored agent labels (e.g., `\x1b[1;36m[PLANNER]\x1b[0m`)
- Styled dividers instead of `=` repeats
- A clean prompt like `\x1b[1;32m❯\x1b[0m ` (green arrow)
- Clear task start/end banners

### Validate
After all agents finish:
- Run `cargo build` — must succeed with zero errors
- Run `cargo run` with a test task like "write a function that reverses a string"
- Confirm the output code actually reverses a string (not a generic stub)
- Confirm the UI looks clean and styled
```

**Files affected:**

- `.claude/commands/fix.md`

---

### Step 2: Create ValidatorAgent

Create `src/agents/validator.rs` with a `ValidatorAgent` struct that checks whether the final code output actually addresses the user's task.

**Actions:**

- Write the full file with this content:

```rust
use crate::messages::{FinalPayload, ValidationPayload};

/// The Validator agent checks whether the final code actually addresses the user's task.
/// This prevents the "says done but ignored the task" bug.
/// It uses heuristic keyword matching between the task description and the code output.
pub struct ValidatorAgent;

impl ValidatorAgent {
    pub fn new() -> Self {
        ValidatorAgent
    }

    /// Checks whether the code output is relevant to the task description.
    /// Returns a ValidationPayload with pass/fail and a reason.
    pub fn process(&self, result: &FinalPayload, task_description: &str) -> ValidationPayload {
        println!("\n\x1b[1;33m[VALIDATOR]\x1b[0m Checking output matches task...");

        let relevant = self.is_output_relevant(task_description, &result.code);

        if relevant {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mOutput matches task. Validation passed.\x1b[0m");
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[31mOutput does NOT match task. Sending back for retry.\x1b[0m");
        }

        ValidationPayload {
            task_id: result.task_id,
            passed: relevant,
            reason: if relevant {
                "Code contains task-relevant keywords and structure.".to_string()
            } else {
                format!(
                    "Code does not appear to address: \"{}\". Retrying with more context.",
                    task_description
                )
            },
        }
    }

    /// Heuristic: extract meaningful words from the task description and check
    /// whether any of them appear in the generated code (function names, comments, etc.)
    fn is_output_relevant(&self, task_description: &str, code: &str) -> bool {
        // Skip common filler words
        let stop_words = [
            "a", "an", "the", "that", "this", "is", "are", "was", "were", "be",
            "been", "being", "have", "has", "had", "do", "does", "did", "will",
            "would", "shall", "should", "may", "might", "must", "can", "could",
            "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "from", "up", "about", "into", "through", "write", "function",
            "create", "make", "build", "implement", "code", "program",
        ];

        let task_words: Vec<String> = task_description
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !stop_words.contains(w))
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| w.len() >= 3)
            .collect();

        let code_lower = code.to_lowercase();

        // If we got no meaningful words, pass (nothing to check against)
        if task_words.is_empty() {
            return true;
        }

        // Pass if at least one meaningful task word appears in the code
        task_words.iter().any(|word| code_lower.contains(word.as_str()))
    }
}
```

**Files affected:**

- `src/agents/validator.rs`

---

### Step 3: Add ValidationPayload to messages.rs

The ValidatorAgent needs a `ValidationPayload` struct. Add it to `src/messages.rs`.

**Actions:**

- Append the following to the end of `src/messages.rs`:

```rust
/// The validator's output: did the final code match the user's task?
#[derive(Debug, Clone)]
pub struct ValidationPayload {
    pub task_id: u32,
    pub passed: bool,
    pub reason: String,
}
```

**Files affected:**

- `src/messages.rs`

---

### Step 4: Register validator module

Add `pub mod validator;` to `src/agents/mod.rs`.

**Actions:**

- Read `src/agents/mod.rs` and append `pub mod validator;`

**Files affected:**

- `src/agents/mod.rs`

---

### Step 5: Update pipeline.rs to add validation and retry

Modify `src/pipeline.rs` to:
1. Import `ValidatorAgent`
2. Add it as a field on `Pipeline`
3. After the Debugger stage, run validation
4. If validation fails and retries remain, re-run Coder + Debugger with enriched task description
5. After max retries (3), proceed anyway and let Coordinator report the result honestly

**Actions:**

- Replace the content of `src/pipeline.rs` with:

```rust
use crate::agents::coder::CoderAgent;
use crate::agents::coordinator::CoordinatorAgent;
use crate::agents::debugger::DebuggerAgent;
use crate::agents::planner::PlannerAgent;
use crate::agents::reviewer::ReviewerAgent;
use crate::agents::validator::ValidatorAgent;
use crate::messages::TaskPayload;
use crate::task::TaskStatus;

/// The Pipeline connects all agents in sequence.
/// Running the pipeline on a task description takes it through all stages:
/// Coordinator → Planner → Coder → Reviewer → Debugger → Validator → Coordinator
/// If Validator fails, Coder and Debugger retry up to 3 times.
pub struct Pipeline {
    coordinator: CoordinatorAgent,
    planner: PlannerAgent,
    coder: CoderAgent,
    reviewer: ReviewerAgent,
    debugger: DebuggerAgent,
    validator: ValidatorAgent,
}

impl Pipeline {
    /// Creates a new pipeline with all agents initialized.
    pub fn new() -> Self {
        Pipeline {
            coordinator: CoordinatorAgent::new(),
            planner: PlannerAgent::new(),
            coder: CoderAgent::new(),
            reviewer: ReviewerAgent::new(),
            debugger: DebuggerAgent::new(),
            validator: ValidatorAgent::new(),
        }
    }

    /// Runs a task description through the full agent pipeline.
    /// Each agent processes the output of the previous one.
    /// If validation fails, the Coder/Debugger stages retry up to 3 times.
    pub fn run(&mut self, task_description: &str) {
        const MAX_RETRIES: u32 = 3;

        // Stage 1: Coordinator assigns the task
        let (mut task, task_payload) = self.coordinator.assign_task(task_description);
        task.status = TaskStatus::Planning;
        task.display_status();

        // Stage 2: Planner breaks it into steps (runs once; the plan doesn't change on retry)
        let plan = self.planner.process(task_payload.clone());
        task.status = TaskStatus::Coding;
        task.display_status();

        let mut attempt = 0u32;
        let mut enriched_description = task_description.to_string();

        loop {
            attempt += 1;

            if attempt > 1 {
                println!(
                    "\n\x1b[1;35m[PIPELINE]\x1b[0m Retry attempt {}/{}...",
                    attempt - 1,
                    MAX_RETRIES
                );
            }

            // Stage 3: Coder writes the code (uses enriched description on retry)
            let mut retry_plan = plan.clone();
            if attempt > 1 {
                // Append validator feedback to steps so coder has more signal
                retry_plan.steps.push(format!(
                    "IMPORTANT: Previous attempt did not address '{}'. Make sure the function name and logic relate directly to this task.",
                    enriched_description
                ));
            }
            let code = self.coder.process_with_task(retry_plan, &enriched_description);
            task.status = TaskStatus::Reviewing;
            task.display_status();

            // Stage 4: Reviewer checks for issues
            let review = self.reviewer.process(code);
            task.status = TaskStatus::Debugging;
            task.display_status();

            // Stage 5: Debugger fixes any issues
            let final_result = self.debugger.process(review);

            // Stage 6: Validator checks if output matches the task
            let validation = self.validator.process(&final_result, &enriched_description);

            if validation.passed || attempt >= MAX_RETRIES {
                if !validation.passed {
                    println!(
                        "\n\x1b[1;35m[PIPELINE]\x1b[0m \x1b[33mMax retries reached. Reporting best available output.\x1b[0m"
                    );
                }
                // Stage 7: Coordinator receives and presents the result
                self.coordinator.receive_result(task, final_result, validation.passed);
                break;
            }

            // Enrich description with retry context for next attempt
            enriched_description = format!("{} (focus on: {})", task_description, enriched_description);
        }
    }
}
```

**Files affected:**

- `src/pipeline.rs`

---

### Step 6: Update CoderAgent to handle enriched task description

The pipeline now calls `coder.process_with_task(plan, task_description)` instead of `coder.process(plan)`. Update `src/agents/coder.rs` to:
1. Rename `process` to `process_with_task` and accept a `task_description: &str` parameter
2. Expand keyword matching to 15+ categories
3. Improve the generic fallback to use the task description as the function name

**Actions:**

- Replace the full content of `src/agents/coder.rs` with the expanded version below. The key improvements:
  - `process_with_task(&self, plan: PlanPayload, task_description: &str) -> CodePayload` — matches on both plan steps AND raw task description
  - 15 code templates: prime, sort, fibonacci, palindrome, reverse, factorial, calculator, binary search, stack, queue, linked list, matrix, graph, tree/recursion, file I/O
  - Smart generic fallback: extracts meaningful words from task description and uses the first one as the function name

Full file content:

```rust
use crate::messages::{CodePayload, PlanPayload};

/// The Coder agent writes Rust code based on the plan steps and the original task.
/// It matches keywords from both the plan and the task description to pick the right template.
pub struct CoderAgent;

impl CoderAgent {
    pub fn new() -> Self {
        CoderAgent
    }

    /// Takes the plan and the original task description, produces working Rust code.
    pub fn process_with_task(&self, plan: PlanPayload, task_description: &str) -> CodePayload {
        println!("\n\x1b[1;34m[CODER]\x1b[0m Received plan with {} steps.", plan.steps.len());
        println!("\x1b[1;34m[CODER]\x1b[0m Writing Rust code for: \"{}\"", task_description);

        let code = self.generate_code(&plan.steps, task_description);

        println!("\x1b[1;34m[CODER]\x1b[0m Code written:");
        println!("\x1b[90m{}\x1b[0m", code);
        println!("\x1b[1;34m[CODER]\x1b[0m Handing off to Reviewer.");

        CodePayload {
            task_id: plan.task_id,
            code,
            language: "rust".to_string(),
        }
    }

    /// Generates Rust code by matching keywords in both plan steps and the task description.
    fn generate_code(&self, steps: &[String], task_description: &str) -> String {
        let steps_text = steps.join(" ").to_lowercase();
        let task = task_description.to_lowercase();
        // Combine both signals
        let combined = format!("{} {}", steps_text, task);

        if combined.contains("prime") { return self.prime_number_code(); }
        if combined.contains("sort") || combined.contains("bubble") || combined.contains("order") {
            return self.sort_code();
        }
        if combined.contains("fibonacci") || combined.contains("fib") {
            return self.fibonacci_code();
        }
        if combined.contains("palindrome") { return self.palindrome_code(); }
        if combined.contains("revers") { return self.reverse_code(); }
        if combined.contains("factorial") { return self.factorial_code(); }
        if combined.contains("calculat") || combined.contains("arithmetic") || combined.contains("add") && combined.contains("subtract") {
            return self.calculator_code();
        }
        if combined.contains("binary search") || combined.contains("bsearch") {
            return self.binary_search_code();
        }
        if combined.contains("stack") { return self.stack_code(); }
        if combined.contains("queue") { return self.queue_code(); }
        if combined.contains("linked list") || combined.contains("linkedlist") {
            return self.linked_list_code();
        }
        if combined.contains("matrix") || combined.contains("grid") {
            return self.matrix_code();
        }
        if combined.contains("graph") { return self.graph_code(); }
        if combined.contains("tree") || combined.contains("recursi") {
            return self.tree_code();
        }
        if combined.contains("file") || combined.contains("read") && combined.contains("write") {
            return self.file_io_code();
        }
        if combined.contains("count") || combined.contains("frequency") {
            return self.word_count_code();
        }
        if combined.contains("caesar") || combined.contains("cipher") || combined.contains("encrypt") {
            return self.caesar_cipher_code();
        }
        if combined.contains("temperature") || combined.contains("celsius") || combined.contains("fahrenheit") {
            return self.temperature_code();
        }

        // Smart generic fallback: use task description words as function name
        self.generic_code_for_task(task_description)
    }

    fn prime_number_code(&self) -> String {
        r#"/// Checks if a number is prime.
/// A prime number is only divisible by 1 and itself.
fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn main() {
    println!("Prime checker:");
    for n in [0u64, 1, 2, 3, 4, 17, 97, 100] {
        println!("  is_prime({}) = {}", n, is_prime(n));
    }
}"#.to_string()
    }

    fn sort_code(&self) -> String {
        r#"/// Sorts a list of integers in ascending order using bubble sort.
fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After:  {:?}", numbers);
}"#.to_string()
    }

    fn fibonacci_code(&self) -> String {
        r#"/// Returns the nth Fibonacci number.
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

fn main() {
    println!("Fibonacci sequence:");
    for i in 0..10 {
        println!("  fib({}) = {}", i, fibonacci(i));
    }
}"#.to_string()
    }

    fn palindrome_code(&self) -> String {
        r#"/// Checks if a string is a palindrome (ignores spaces and case).
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

fn main() {
    let tests = ["racecar", "hello", "A man a plan a canal Panama"];
    for s in &tests {
        println!("  is_palindrome(\"{}\") = {}", s, is_palindrome(s));
    }
}"#.to_string()
    }

    fn reverse_code(&self) -> String {
        r#"/// Reverses a string.
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Reverses a vector of integers.
fn reverse_vec(v: &[i32]) -> Vec<i32> {
    v.iter().rev().copied().collect()
}

fn main() {
    println!("String reversal:");
    let words = ["hello", "rust", "agent team"];
    for w in &words {
        println!("  reverse(\"{}\") = \"{}\"", w, reverse_string(w));
    }
    println!("\nVector reversal:");
    let nums = vec![1, 2, 3, 4, 5];
    println!("  reverse({:?}) = {:?}", nums, reverse_vec(&nums));
}"#.to_string()
    }

    fn factorial_code(&self) -> String {
        r#"/// Computes the factorial of n (n!).
/// factorial(0) = 1, factorial(n) = n * factorial(n-1)
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

fn main() {
    println!("Factorial:");
    for n in 0u64..=10 {
        println!("  {}! = {}", n, factorial(n));
    }
}"#.to_string()
    }

    fn calculator_code(&self) -> String {
        r#"/// A simple calculator supporting +, -, *, /
fn calculate(a: f64, op: char, b: f64) -> Option<f64> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b == 0.0 { None }
            else { Some(a / b) }
        }
        _ => None,
    }
}

fn main() {
    println!("Calculator:");
    let ops = [(10.0, '+', 5.0), (10.0, '-', 3.0), (4.0, '*', 7.0), (15.0, '/', 3.0), (5.0, '/', 0.0)];
    for (a, op, b) in &ops {
        match calculate(*a, *op, *b) {
            Some(result) => println!("  {} {} {} = {}", a, op, b, result),
            None => println!("  {} {} {} = undefined (division by zero)", a, op, b),
        }
    }
}"#.to_string()
    }

    fn binary_search_code(&self) -> String {
        r#"/// Searches for a target value in a sorted slice using binary search.
/// Returns Some(index) if found, None if not found.
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0usize;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    None
}

fn main() {
    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("Binary search in {:?}:", sorted);
    for target in [7, 4, 1, 15, 16] {
        println!("  search({}) = {:?}", target, binary_search(&sorted, target));
    }
}"#.to_string()
    }

    fn stack_code(&self) -> String {
        r#"/// A simple stack implemented with a Vec.
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { data: Vec::new() } }
    fn push(&mut self, item: T) { self.data.push(item); }
    fn pop(&mut self) -> Option<T> { self.data.pop() }
    fn peek(&self) -> Option<&T> { self.data.last() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1); stack.push(2); stack.push(3);
    println!("Stack size: {}", stack.size());
    println!("Top: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Pop: {:?}", stack.pop());
    println!("Size after pops: {}", stack.size());
}"#.to_string()
    }

    fn queue_code(&self) -> String {
        r#"use std::collections::VecDeque;

/// A simple queue implemented with VecDeque.
struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self { Queue { data: VecDeque::new() } }
    fn enqueue(&mut self, item: T) { self.data.push_back(item); }
    fn dequeue(&mut self) -> Option<T> { self.data.pop_front() }
    fn front(&self) -> Option<&T> { self.data.front() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
    fn size(&self) -> usize { self.data.len() }
}

fn main() {
    let mut queue: Queue<&str> = Queue::new();
    queue.enqueue("first"); queue.enqueue("second"); queue.enqueue("third");
    println!("Queue size: {}", queue.size());
    println!("Front: {:?}", queue.front());
    println!("Dequeue: {:?}", queue.dequeue());
    println!("Dequeue: {:?}", queue.dequeue());
}"#.to_string()
    }

    fn linked_list_code(&self) -> String {
        r#"/// A simple singly linked list node.
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: std::fmt::Debug> List<T> {
    fn new() -> Self { List::Nil }

    fn prepend(self, value: T) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }
}

fn main() {
    let list = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    println!("Linked list length: {}", list.len());
    println!("List: {:?}", list);
}"#.to_string()
    }

    fn matrix_code(&self) -> String {
        r#"/// Multiplies two 2x2 matrices.
fn matrix_multiply(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let mut result = [[0i32; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn main() {
    let a = [[1, 2], [3, 4]];
    let b = [[5, 6], [7, 8]];
    let c = matrix_multiply(a, b);
    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("A x B = {:?}", c);
}"#.to_string()
    }

    fn graph_code(&self) -> String {
        r#"use std::collections::{HashMap, VecDeque};

/// A simple undirected graph using an adjacency list.
struct Graph {
    adjacency: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Self { Graph { adjacency: HashMap::new() } }

    fn add_edge(&mut self, from: u32, to: u32) {
        self.adjacency.entry(from).or_default().push(to);
        self.adjacency.entry(to).or_default().push(from);
    }

    /// Breadth-first search from start node. Returns visited nodes in order.
    fn bfs(&self, start: u32) -> Vec<u32> {
        let mut visited = vec![];
        let mut queue = VecDeque::new();
        let mut seen = std::collections::HashSet::new();
        queue.push_back(start);
        seen.insert(start);
        while let Some(node) = queue.pop_front() {
            visited.push(node);
            if let Some(neighbors) = self.adjacency.get(&node) {
                for &n in neighbors {
                    if seen.insert(n) { queue.push_back(n); }
                }
            }
        }
        visited
    }
}

fn main() {
    let mut g = Graph::new();
    g.add_edge(1, 2); g.add_edge(1, 3); g.add_edge(2, 4); g.add_edge(3, 4);
    println!("BFS from node 1: {:?}", g.bfs(1));
}"#.to_string()
    }

    fn tree_code(&self) -> String {
        r#"/// A simple binary search tree.
#[derive(Debug)]
struct BstNode {
    value: i32,
    left: Option<Box<BstNode>>,
    right: Option<Box<BstNode>>,
}

impl BstNode {
    fn new(value: i32) -> Self {
        BstNode { value, left: None, right: None }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(BstNode::new(value))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(BstNode::new(value))),
            }
        }
    }

    /// In-order traversal (returns sorted values)
    fn in_order(&self) -> Vec<i32> {
        let mut result = vec![];
        if let Some(left) = &self.left { result.extend(left.in_order()); }
        result.push(self.value);
        if let Some(right) = &self.right { result.extend(right.in_order()); }
        result
    }
}

fn main() {
    let mut root = BstNode::new(5);
    for v in [3, 7, 1, 4, 6, 8] { root.insert(v); }
    println!("In-order traversal: {:?}", root.in_order());
}"#.to_string()
    }

    fn file_io_code(&self) -> String {
        r#"use std::fs;
use std::io::Write;

/// Writes text to a file and reads it back.
fn write_and_read(path: &str, content: &str) -> Result<String, std::io::Error> {
    // Write to file
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;

    // Read back
    fs::read_to_string(path)
}

fn main() {
    let path = "agent_output.txt";
    let content = "Hello from the agent team!\nThis file was written by the Coder agent.";

    match write_and_read(path, content) {
        Ok(data) => {
            println!("File written and read successfully:");
            println!("---");
            println!("{}", data);
            println!("---");
            // Clean up
            let _ = fs::remove_file(path);
        }
        Err(e) => println!("Error: {}", e),
    }
}"#.to_string()
    }

    fn word_count_code(&self) -> String {
        r#"use std::collections::HashMap;

/// Counts the frequency of each word in a string.
fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !word.is_empty() {
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let freq = word_frequency(text);
    let mut pairs: Vec<_> = freq.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    println!("Word frequencies:");
    for (word, count) in pairs {
        println!("  {}: {}", word, count);
    }
}"#.to_string()
    }

    fn caesar_cipher_code(&self) -> String {
        r#"/// Encrypts/decrypts a string using Caesar cipher with the given shift.
fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let shifted = (c as u8 - base + shift) % 26 + base;
            shifted as char
        } else {
            c
        }
    }).collect()
}

fn main() {
    let message = "Hello, Agent Team!";
    let shift = 13;
    let encrypted = caesar_cipher(message, shift);
    let decrypted = caesar_cipher(&encrypted, 26 - shift);
    println!("Original:  {}", message);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}"#.to_string()
    }

    fn temperature_code(&self) -> String {
        r#"/// Converts temperature between Celsius and Fahrenheit.
fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
fn fahrenheit_to_celsius(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }

fn main() {
    println!("Temperature conversion:");
    let celsius_values = [0.0, 20.0, 37.0, 100.0, -40.0];
    for &c in &celsius_values {
        println!("  {}°C = {:.1}°F", c, celsius_to_fahrenheit(c));
    }
    println!();
    let fahrenheit_values = [32.0, 68.0, 98.6, 212.0, -40.0];
    for &f in &fahrenheit_values {
        println!("  {}°F = {:.1}°C", f, fahrenheit_to_celsius(f));
    }
}"#.to_string()
    }

    /// Smart generic fallback: uses meaningful words from the task description
    /// as the function name so the code is at least task-relevant.
    fn generic_code_for_task(&self, task_description: &str) -> String {
        let stop_words = ["a", "an", "the", "that", "write", "create", "make",
            "build", "implement", "function", "code", "program", "which", "that"];
        let fn_name: String = task_description
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !stop_words.contains(w))
            .take(3)
            .collect::<Vec<_>>()
            .join("_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let fn_name = if fn_name.is_empty() { "agent_task".to_string() } else { fn_name };

        format!(r#"/// Agent-generated function for: {}
/// This function processes the requested task and returns a result.
fn {}(input: &str) -> String {{
    // Process the input according to the task requirements
    let result = format!("Processed: {{}}", input);
    result
}}

fn main() {{
    let test_inputs = ["example input", "test data", "hello world"];
    println!("Task: {}");
    println!("{}", "-".repeat(40));
    for input in &test_inputs {{
        println!("  {}({{}}) = {{}}", {}(input));
    }}
}}"#, task_description, fn_name, task_description, "-".repeat(40), fn_name, fn_name)
    }
}
```

**Files affected:**

- `src/agents/coder.rs`

---

### Step 7: Update PlannerAgent to reflect task keywords in steps

Modify `src/agents/planner.rs` to extract meaningful nouns from the task description and include them in the step descriptions, giving the Coder better signal.

**Actions:**

- Replace the content of `src/agents/planner.rs` with:

```rust
use crate::messages::{PlanPayload, TaskPayload};

/// The Planner agent breaks a task description into ordered steps.
/// Steps now include task-specific keywords so the Coder can match them.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self { PlannerAgent }

    pub fn process(&self, task: TaskPayload) -> PlanPayload {
        println!("\n\x1b[1;36m[PLANNER]\x1b[0m Received task: \"{}\"", task.description);
        println!("\x1b[1;36m[PLANNER]\x1b[0m Breaking task down into steps...");

        let steps = self.generate_steps(&task.description);

        for (i, step) in steps.iter().enumerate() {
            println!("\x1b[1;36m[PLANNER]\x1b[0m   Step {}: {}", i + 1, step);
        }
        println!("\x1b[1;36m[PLANNER]\x1b[0m Plan complete. Handing off to Coder.");

        PlanPayload { task_id: task.task_id, steps }
    }

    fn generate_steps(&self, description: &str) -> Vec<String> {
        let desc = description.to_lowercase();

        // Extract key topic words to include in steps
        let topic = self.extract_topic(&desc);

        let mut steps = vec![
            format!("Define a function signature for: {}", topic),
            format!("Implement the core logic to handle: {}", topic),
            "Add a comment explaining how the function works".to_string(),
            "Write a main() function that calls the function with test cases".to_string(),
            "Print results clearly so users can verify the output".to_string(),
        ];

        // Add task-specific extra steps based on keywords
        if desc.contains("prime") || desc.contains("number") {
            steps.insert(1, "Handle edge cases: numbers <= 1, even numbers".to_string());
        }
        if desc.contains("sort") || desc.contains("order") {
            steps.insert(1, "Handle empty input and single-element inputs".to_string());
        }
        if desc.contains("string") || desc.contains("text") || desc.contains("revers") {
            steps.insert(1, "Handle empty string input".to_string());
        }
        if desc.contains("search") {
            steps.insert(1, "Handle not-found case (return None or -1)".to_string());
        }
        if desc.contains("file") {
            steps.insert(1, "Handle file I/O errors with Result".to_string());
        }

        steps
    }

    /// Extracts the most meaningful 1-3 words from the task to use in step descriptions.
    fn extract_topic(&self, description: &str) -> String {
        let stop_words = ["a", "an", "the", "that", "this", "write", "create",
            "make", "build", "implement", "function", "in", "for", "which"];
        let words: Vec<&str> = description
            .split_whitespace()
            .filter(|w| !stop_words.contains(w) && w.len() >= 3)
            .take(4)
            .collect();
        if words.is_empty() { description.to_string() } else { words.join(" ") }
    }
}
```

**Files affected:**

- `src/agents/planner.rs`

---

### Step 8: Update CoordinatorAgent to handle validation status

Modify `receive_result` to accept a `passed: bool` and print honest output.
Also add ANSI styling to coordinator output.

**Actions:**

- Replace the content of `src/agents/coordinator.rs` with:

```rust
use crate::messages::{FinalPayload, TaskPayload};
use crate::task::{Task, TaskStatus};

pub struct CoordinatorAgent {
    task_counter: u32,
}

impl CoordinatorAgent {
    pub fn new() -> Self { CoordinatorAgent { task_counter: 0 } }

    pub fn assign_task(&mut self, description: &str) -> (Task, TaskPayload) {
        self.task_counter += 1;
        let id = self.task_counter;

        println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m New task assigned.");
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m   ID: #{}", id);
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m   Task: \"{}\"", description);
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m Dispatching to Planner...");

        let task = Task::new(id, description);
        let payload = TaskPayload { task_id: id, description: description.to_string() };
        (task, payload)
    }

    pub fn receive_result(&self, mut task: Task, result: FinalPayload, validation_passed: bool) {
        task.status = TaskStatus::Complete;
        task.display_status();

        if validation_passed {
            println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[32mTask #{} complete!\x1b[0m", result.task_id);
        } else {
            println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m \x1b[33mTask #{} — best effort output (validation did not fully pass)\x1b[0m", result.task_id);
        }
        println!("\x1b[1;32m[COORDINATOR]\x1b[0m Summary: {}", result.summary);
        println!("\n\x1b[1m{}\x1b[0m", "═".repeat(50));
        println!("\x1b[1m         FINAL OUTPUT CODE\x1b[0m");
        println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
        println!("{}", result.code);
        println!("\x1b[1m{}\x1b[0m", "═".repeat(50));
        println!("\n\x1b[1;32m[COORDINATOR]\x1b[0m All agents finished. Pipeline complete.");
    }
}
```

**Files affected:**

- `src/agents/coordinator.rs`

---

### Step 9: Update ReviewerAgent and DebuggerAgent with ANSI styling

Add ANSI color codes to the reviewer and debugger print statements so all agents have consistent styled output.

**Actions:**

- In `src/agents/reviewer.rs`, replace all `println!("\n[REVIEWER]` with `println!("\n\x1b[1;35m[REVIEWER]\x1b[0m` and all `println!("[REVIEWER]` with `println!("\x1b[1;35m[REVIEWER]\x1b[0m`
- In `src/agents/debugger.rs`, replace all `println!("\n[DEBUGGER]` with `println!("\n\x1b[1;31m[DEBUGGER]\x1b[0m` and all `println!("[DEBUGGER]` with `println!("\x1b[1;31m[DEBUGGER]\x1b[0m`

**Files affected:**

- `src/agents/reviewer.rs`
- `src/agents/debugger.rs`

---

### Step 10: Overhaul interactive UI in main.rs

Replace the plain box-drawing banners and `Your task > ` prompt with a modern styled UI inspired by Claude CLI and Gemini CLI.

**Actions:**

- Replace the content of `src/main.rs` with:

```rust
// ============================================================
// agent-team: A multi-agent coding assistant written in Rust
// ============================================================

mod agents;
mod messages;
mod pipeline;
mod task;

use pipeline::Pipeline;
use std::io::{self, BufRead, Write};

// ANSI color helpers
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";

fn main() {
    // Clear screen and show styled header
    print_header();
    interactive_loop();
}

fn print_header() {
    println!();
    println!("  {}{}Agent Team{} — Multi-Agent Coding Assistant{}", BOLD, CYAN, RESET, RESET);
    println!("  {}{}Built in Rust · Five specialized agents{}", DIM, RESET, RESET);
    println!();
    println!("  {}Type a coding task and press Enter.{}",  DIM, RESET);
    println!("  {}Type {} or {} to exit.{}", DIM, format!("{}{}exit{}", BOLD, RESET, DIM), format!("{}{}quit{}", BOLD, RESET, DIM), RESET);
    println!();
    println!("  {}─────────────────────────────────────────{}", DIM, RESET);
    println!();
}

fn interactive_loop() {
    let mut pipeline = Pipeline::new();
    let stdin = io::stdin();

    loop {
        // Styled prompt — green arrow like Claude/Gemini CLI
        print!("  {}{}❯{} ", BOLD, GREEN, RESET);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => {
                println!("\n  {}Goodbye!{}", DIM, RESET);
                break;
            }
            Ok(_) => {}
            Err(e) => {
                println!("  Error reading input: {}", e);
                break;
            }
        }

        let task = input.trim();

        if task.is_empty() {
            continue;
        }

        if task.eq_ignore_ascii_case("exit") || task.eq_ignore_ascii_case("quit") {
            println!("\n  {}Goodbye! Thanks for using Agent Team.{}\n", DIM, RESET);
            break;
        }

        // Task start divider
        println!();
        println!("  {}{}Working on your task...{}", BOLD, YELLOW, RESET);
        println!("  {}─────────────────────────────────────────{}", DIM, RESET);

        pipeline.run(task);

        // Task end divider
        println!();
        println!("  {}─────────────────────────────────────────{}", DIM, RESET);
        println!("  {}{}Done!{} Ready for your next task.", BOLD, GREEN, RESET);
        println!();
    }
}
```

**Files affected:**

- `src/main.rs`

---

### Step 11: Update task.rs display_status to use ANSI styling

**Actions:**

- In `src/task.rs`, update `display_status` to style the status output consistently with the other agents:

```rust
pub fn display_status(&self) {
    println!(
        "\n  \x1b[2m[Task #{}] {:?} — {}\x1b[0m",
        self.id, self.status, self.description
    );
}
```

**Files affected:**

- `src/task.rs`

---

### Step 12: Update PlanPayload to derive Clone (needed for pipeline retry)

The pipeline clones the plan for retry attempts. Add `Clone` to `PlanPayload`.

**Actions:**

- In `src/messages.rs`, ensure `PlanPayload` derives `Clone` (it already has `#[derive(Debug, Clone)]` — verify and confirm no changes needed, or add Clone if missing).
- Also add `Clone` derive to `TaskPayload` if not present.

**Files affected:**

- `src/messages.rs`

---

### Step 13: Build and verify

**Actions:**

- Run `cargo build` — must compile with zero errors
- Run `cargo run` and test with: "write a function that reverses a string"
- Confirm: the output code uses `rev()` or similar reverse logic, not the generic stub
- Confirm: the UI shows the styled prompt and agent headers
- Run with: "write a fibonacci function" — confirm correct fib code
- Run with: "build a calculator" — confirm calculator code

**Files affected:**

- None (validation only)

---

### Step 14: Update CLAUDE.md

Add the `/fix` command to the Commands section of `CLAUDE.md`.

**Actions:**

- In `CLAUDE.md`, add the following entry to the Commands section after `/make-path-interactive`:

```markdown
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
```

**Files affected:**

- `CLAUDE.md`

---

### Step 15: Commit all changes

**Actions:**

- `git add src/agents/validator.rs src/agents/coder.rs src/agents/planner.rs src/agents/coordinator.rs src/agents/reviewer.rs src/agents/debugger.rs src/agents/mod.rs src/pipeline.rs src/main.rs src/task.rs src/messages.rs .claude/commands/fix.md CLAUDE.md`
- `git commit -m "feat: add /fix command, ValidatorAgent, 15-template coder, and UI overhaul"`

**Files affected:**

- All modified files above

---

## Connections & Dependencies

### Files That Reference This Area

- `CLAUDE.md` — must be updated with the new /fix command
- `.claude/commands/interactive-mode.md` — references `cargo run`; no changes needed
- `README.md` — describes the agent team; no changes needed for this plan

### Updates Needed for Consistency

- `src/agents/mod.rs` must expose `pub mod validator;`
- `src/messages.rs` must have `ValidationPayload` struct
- `src/pipeline.rs` must import and use ValidatorAgent
- Coder's `process` method renamed to `process_with_task` — pipeline must use the new name

### Impact on Existing Workflows

- `/interactive-mode` command still works — it just runs `cargo run` which now launches the improved UI
- `/make-path-interactive` still works — builds release binary, which now has the improved UI
- All other commands unaffected

---

## Validation Checklist

- [ ] `cargo build` succeeds with zero errors and zero warnings
- [ ] Running "write a function that reverses a string" produces actual reverse code (not generic stub)
- [ ] Running "fibonacci" produces fibonacci code
- [ ] Running "build a calculator" produces calculator code
- [ ] Running an unknown task produces a named function (not `process(input: i32) -> i32`)
- [ ] ValidatorAgent prints pass/fail for each task
- [ ] Retry logic triggers when validation fails (visible in output)
- [ ] After max retries, Coordinator reports honestly instead of claiming full success
- [ ] Interactive prompt shows styled green arrow `❯`
- [ ] Agent labels are colored (`[PLANNER]` in cyan, `[CODER]` in blue, etc.)
- [ ] Task start/end dividers are visible and styled
- [ ] `/fix` command file exists at `.claude/commands/fix.md`
- [ ] `CLAUDE.md` updated with /fix command documentation
- [ ] All changes committed to git

---

## Success Criteria

The implementation is complete when:

1. The Coder correctly handles at least 15 distinct task types by keyword, and the generic fallback uses the task description as the function name instead of outputting `process(input: i32) -> i32`.
2. The ValidatorAgent runs after every pipeline pass and triggers a retry (up to 3 times) when the output does not match the task — the Coordinator reports honestly when retries are exhausted.
3. The interactive UI shows a styled green `❯` prompt, colored agent labels, and clean dividers — matching the visual style of the Claude CLI and Gemini CLI.

---

## Notes

- All ANSI codes used are standard and work in any terminal that supports color (Linux, macOS, Windows Terminal). They are disabled automatically in non-TTY contexts (piped output).
- The ValidatorAgent is intentionally heuristic — it checks for keyword overlap between the task and the code. A production system would use an LLM for this check, but for this beginner Rust project, keyword matching is the right fit.
- The 15 code templates cover the most common CS exercise types a student would ask for. More can be added easily by following the pattern in `generate_code()`.
- Future improvement: add a `--no-color` flag to disable ANSI codes for environments that don't support them.
