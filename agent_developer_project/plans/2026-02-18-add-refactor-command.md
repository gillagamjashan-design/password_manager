# Plan: Add /refactor Command — Delete All AI APIs, Give Each Agent a Pure Rust Brain

**Created:** 2026-02-18
**Status:** Draft
**Request:** Add a /refactor command that deletes ALL external AI API code (OpenAI, DeepSeek, Anthropic — everything), removes all API key requirements, then spawns 4 Claude Code agents (1 Opus 4.5 + 3 Sonnet 4.5) to rewrite each Rust agent with its own specialized built-in brain written in pure Rust.

---

## Overview

### What This Plan Accomplishes

The `/refactor` command performs a full wipe of all external AI dependencies and rewrites every agent with a smart, specialized brain built directly in Rust — no HTTP calls, no API keys, no external services required. Four Claude Code agents (1 Opus 4.5 architect + 3 Sonnet 4.5 workers) do the work. When done they report completion and append the new features to the oldest plan.

### Why This Matters

The project currently requires three API keys to function. After this refactor the project runs completely standalone — any user can clone it and run it immediately with zero setup. Each agent gets a purpose-built Rust brain tuned to its specialty: the Planner uses pattern analysis to generate smart steps, the Coder generates structured Rust code from task keywords, the Reviewer applies static analysis rules, the Debugger applies known fix patterns, and the Validator checks correctness with keyword and structure checks.

---

## Current State

### Relevant Existing Structure

- `src/ai_client.rs` — three provider functions calling OpenAI, DeepSeek, Anthropic over HTTP
- `src/agents/coder.rs` — imports `call_gpt`
- `src/agents/debugger.rs` — imports `call_deepseek`
- `src/agents/planner.rs` — imports `call_claude`
- `src/agents/reviewer.rs` — imports `call_claude`
- `src/agents/validator.rs` — imports `call_claude`
- `src/agents/coordinator.rs` — no AI calls
- `Cargo.toml` — has `reqwest`, `serde`, `serde_json` (only there for API calls — can be removed)
- `.claude/commands/` — slash command definitions
- `plans/2026-02-17-rust-multi-agent-coding-system.md` — oldest plan

### Gaps or Problems Being Addressed

- Requires 3 API keys — blocks any user who doesn't have them
- Depends on external services being up and having credit
- `src/ai_client.rs` is dead weight once external calls are gone
- `reqwest`, `serde`, `serde_json` are unnecessary dependencies once HTTP is removed
- No `/refactor` command exists yet

---

## Proposed Changes

### Summary of Changes

- Add `.claude/commands/refactor.md` — new slash command definition
- **Delete** `src/ai_client.rs` entirely — no replacement, file is gone
- **Remove** `mod ai_client;` from `src/main.rs`
- **Remove** `reqwest`, `serde`, `serde_json` from `Cargo.toml`
- **Rewrite** all 5 AI-using agent files with pure Rust brains (no imports from ai_client)
- Update `src/agents/coordinator.rs` — remove stale brain label text
- Update `CLAUDE.md` — add /refactor command, note zero API keys needed
- Append feature summary to `plans/2026-02-17-rust-multi-agent-coding-system.md`
- Commit all changes

### New Files to Create

| File Path | Purpose |
| --- | --- |
| `.claude/commands/refactor.md` | Slash command — spawns 4 Claude Code agents to do the rewrite |

### Files to Rewrite

| File Path | Changes |
| --- | --- |
| `src/agents/planner.rs` | Pure Rust brain — smart step generation from task keyword analysis |
| `src/agents/coder.rs` | Pure Rust brain — structured Rust code generation from task type + keywords |
| `src/agents/reviewer.rs` | Pure Rust brain — static analysis rules (comments, main(), unwrap, safety) |
| `src/agents/debugger.rs` | Pure Rust brain — known fix patterns applied to review issues |
| `src/agents/validator.rs` | Pure Rust brain — keyword + structure checks to confirm task match |
| `src/agents/coordinator.rs` | Remove brain label, keep task management logic |
| `src/main.rs` | Remove `mod ai_client;` line |
| `Cargo.toml` | Remove `reqwest`, `serde`, `serde_json` |
| `CLAUDE.md` | Add /refactor section, note no API keys needed |
| `plans/2026-02-17-rust-multi-agent-coding-system.md` | Append feature summary |

### Files to Delete

| File | Reason |
| --- | --- |
| `src/ai_client.rs` | All external AI code gone — file serves no purpose |

---

## Design Decisions

### Key Decisions Made

1. **Pure Rust brains, zero dependencies**: No HTTP client, no JSON, no API keys. The agents reason using Rust logic — pattern matching, keyword extraction, rule sets, and code templates. This makes the project fully standalone.

2. **Each agent's brain is specialized to its role**: The Planner knows how to break tasks into steps using task-type detection. The Coder knows how to generate Rust code structures. The Reviewer knows common Rust code quality rules. The Debugger knows how to apply fix patterns. The Validator knows how to check correctness.

3. **Remove reqwest, serde, serde_json from Cargo.toml**: These were only there for HTTP calls. Removing them keeps the project lean and fast to compile.

4. **Delete ai_client.rs, don't replace it**: No new ai_client.rs is written. The agents no longer have a shared AI layer — each owns its own logic directly.

5. **4 Claude Code agents spawn the work**: 1 Opus 4.5 (architect, plans the brains and reviews the final output) + 3 Sonnet 4.5 (workers: one handles Cargo/main cleanup, one rewrites all agents, one validates and finalizes).

6. **Remove brain label from coordinator**: Coordinator never had an AI brain. The label "Brain: Claude (Architecture)" was misleading — it's removed.

### Alternatives Considered

- **Keep AI calls but make them optional**: Rejected — the user explicitly wants the API key requirement deleted entirely.
- **Keep reqwest for other potential future use**: Rejected — don't add code for hypothetical future needs. Remove what isn't used.

### Open Questions

None.

---

## Step-by-Step Tasks

### Step 1: Create `.claude/commands/refactor.md`

**Full file content:**

```markdown
# Refactor

Delete ALL external AI API code and API key requirements. Rewrite every agent with a specialized pure-Rust brain. No API keys needed after this runs.

## What This Does

1. Deletes `src/ai_client.rs` entirely
2. Removes `reqwest`, `serde`, `serde_json` from `Cargo.toml`
3. Removes `mod ai_client;` from `src/main.rs`
4. Rewrites all 5 AI-using agents with smart pure-Rust brains
5. Verifies the project compiles and runs
6. Updates CLAUDE.md and appends feature summary to the oldest plan
7. Commits all changes
8. Reports done

## No Setup Required After This

Zero API keys. Zero external services. Just `cargo run`.

## Run

Spawn 4 Claude agents in parallel:

### Agent 1 (Opus 4.5) — Architect
Read all files in `src/`, `src/agents/`, and `Cargo.toml`.
Understand the full pipeline: how tasks flow from Coordinator → Planner → Coder → Reviewer → Debugger → Validator.
Plan what each agent's pure-Rust brain should do — what inputs it gets, what logic it applies, what it outputs.
Review Agent 3's agent rewrites when done and confirm they are correct and complete.

### Agent 2 (Sonnet 4.5) — Cleanup
1. Delete `src/ai_client.rs`
2. In `src/main.rs`, remove the line `mod ai_client;`
3. In `Cargo.toml`, remove these three lines:
   - `reqwest = { version = "0.12", features = ["blocking", "json"] }`
   - `serde = { version = "1", features = ["derive"] }`
   - `serde_json = "1"`
4. Report "Agent 2 done — ai_client deleted, dependencies cleaned."

### Agent 3 (Sonnet 4.5) — Rewrite All Agents
Rewrite all 5 AI-using agent files with pure-Rust brains. Keep the same struct names, method names, and message type signatures — only the internal logic changes.

**planner.rs** — Smart step generation:
- Detect the task type from keywords (sort, reverse, search, parse, read, write, hash, format, count, filter, etc.)
- Return a tailored list of 4-6 implementation steps specific to that task type
- Fallback: generic steps if no keyword matches
- Remove brain label from prints, or update to: `"· Brain: Built-in (Architecture)"`

**coder.rs** — Structured Rust code generation:
- Detect task type from the description and plan steps
- Generate a complete, working Rust code template for that task type (real function logic, not just a stub)
- Include a main() with example usage
- Include comments
- Remove brain label or update to: `"· Brain: Built-in (Coding)"`

**reviewer.rs** — Static analysis:
- Check for: missing `fn main()`, missing comments (`//` or `///`), excessive `.unwrap()` calls (more than 3), empty functions, obvious placeholders like `todo!()` or `unimplemented!()`
- If none found: approve
- If found: list issues
- Remove brain label or update to: `"· Brain: Built-in (Security & Docs)"`

**debugger.rs** — Fix pattern application:
- For each issue from the reviewer, apply a known fix:
  - Missing main() → append a main() block
  - Missing comments → insert doc comments above each `fn`
  - Excessive unwrap → add a note comment suggesting error handling
- Return the fixed code with a summary
- Remove brain label or update to: `"· Brain: Built-in (Debugging)"`

**validator.rs** — Task match check:
- Extract meaningful keywords from the task description (skip stop words)
- Check if the code contains those keywords (function names, variable names, comments)
- Check if a `fn main()` is present
- Pass if both checks succeed, fail otherwise with a clear reason
- Remove brain label or update to: `"· Brain: Built-in (Testing)"`

**coordinator.rs** — Remove the brain label print line entirely (it never had one).

Report "Agent 3 done — all agents rewritten with pure-Rust brains."

### Agent 4 (Sonnet 4.5) — Validate and Finalize
After Agents 2 and 3 finish:

1. Run `cargo build` — must succeed with zero errors. Fix any errors found.
2. Confirm no imports from `ai_client` remain anywhere in `src/agents/`
3. Confirm `Cargo.toml` has no `reqwest`, `serde`, `serde_json`
4. Update `CLAUDE.md`: add /refactor to Commands section, note zero API keys required
5. Append the refactor feature summary to `plans/2026-02-17-rust-multi-agent-coding-system.md`
6. Stage and commit all changes: `git add -A && git commit -m "refactor: remove all AI APIs — agents now run on pure-Rust brains, zero API keys needed"`
7. Report "Done — all agents run on built-in Rust brains. No API keys needed. Just cargo run."
```

**Files affected:** `.claude/commands/refactor.md` (new)

---

### Step 2: Delete `src/ai_client.rs`

**Actions:**
- Delete the file entirely using bash: `rm src/ai_client.rs`

**Files affected:** `src/ai_client.rs` (deleted)

---

### Step 3: Update `Cargo.toml` — Remove HTTP Dependencies

**Actions:**
- Remove `reqwest = { version = "0.12", features = ["blocking", "json"] }`
- Remove `serde = { version = "1", features = ["derive"] }`
- Remove `serde_json = "1"`
- The `[dependencies]` section becomes empty (or can be removed)

**New full content for `Cargo.toml`:**

```toml
[package]
name = "agent-team"
version = "0.1.0"
edition = "2021"
description = "A multi-agent coding assistant built in Rust"
authors = ["Jashan"]

[[bin]]
name = "agent-team"
path = "src/main.rs"

[dependencies]
```

**Files affected:** `Cargo.toml`

---

### Step 4: Update `src/main.rs` — Remove `mod ai_client`

**Actions:**
- Remove the line `mod ai_client;`

**Files affected:** `src/main.rs`

---

### Step 5: Rewrite `src/agents/planner.rs` — Pure Rust Architecture Brain

The Planner detects the task type from keywords and returns a tailored set of implementation steps.

**New full content:**

```rust
use crate::messages::{PlanPayload, TaskPayload};

/// The Planner agent breaks a task into ordered implementation steps.
/// Brain: Built-in — detects task type from keywords and returns tailored steps.
pub struct PlannerAgent;

impl PlannerAgent {
    pub fn new() -> Self { PlannerAgent }

    pub fn process(&self, task: TaskPayload) -> PlanPayload {
        println!("\n\x1b[1;36m[PLANNER]\x1b[0m Received task: \"{}\"", task.description);
        println!("\x1b[1;36m[PLANNER]\x1b[0m \x1b[2m· Brain: Built-in (Architecture)\x1b[0m");
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

        if desc.contains("sort") || desc.contains("order") {
            vec![
                "Define a function that takes a Vec<i32> (or generic slice) as input".to_string(),
                "Use Rust's built-in .sort() or implement a comparison-based sort algorithm".to_string(),
                "Handle edge cases: empty slice, single element".to_string(),
                "Add comments explaining the sort logic and time complexity".to_string(),
                "Write a main() function that creates a test Vec, sorts it, and prints before/after".to_string(),
            ]
        } else if desc.contains("revers") {
            vec![
                "Define a function that takes a string or Vec as input".to_string(),
                "Use Rust's .rev() iterator or implement manual reversal with index swapping".to_string(),
                "Return the reversed value as the same type".to_string(),
                "Add comments explaining the reversal logic".to_string(),
                "Write a main() that tests the function with example inputs and prints results".to_string(),
            ]
        } else if desc.contains("search") || desc.contains("find") {
            vec![
                "Define a function that takes a collection and a target value as parameters".to_string(),
                "Implement the search algorithm (linear scan or binary search if sorted)".to_string(),
                "Return an Option<usize> — Some(index) if found, None if not".to_string(),
                "Add comments explaining the search strategy".to_string(),
                "Write a main() that tests found and not-found cases and prints results".to_string(),
            ]
        } else if desc.contains("fibonacci") || desc.contains("fib") {
            vec![
                "Define a function that takes n: u64 and returns the nth Fibonacci number".to_string(),
                "Implement iteratively (not recursively) to avoid stack overflow on large n".to_string(),
                "Handle base cases: n=0 returns 0, n=1 returns 1".to_string(),
                "Add comments explaining the iterative approach".to_string(),
                "Write a main() that prints the first 10 Fibonacci numbers".to_string(),
            ]
        } else if desc.contains("factorial") {
            vec![
                "Define a function that takes n: u64 and returns n factorial".to_string(),
                "Implement iteratively using a running product".to_string(),
                "Handle edge case: 0! = 1".to_string(),
                "Add comments explaining the factorial logic".to_string(),
                "Write a main() that prints factorials for 0 through 10".to_string(),
            ]
        } else if desc.contains("prime") {
            vec![
                "Define a function that takes n: u64 and returns true if n is prime".to_string(),
                "Implement trial division up to sqrt(n) for efficiency".to_string(),
                "Handle edge cases: 0 and 1 are not prime".to_string(),
                "Add comments explaining the primality test logic".to_string(),
                "Write a main() that prints all primes up to 50".to_string(),
            ]
        } else if desc.contains("read") && desc.contains("file") {
            vec![
                "Use std::fs::read_to_string to read the file contents into a String".to_string(),
                "Handle the Result with proper error handling using match or ?".to_string(),
                "Process the file contents as needed by the task".to_string(),
                "Add comments explaining file I/O and error handling".to_string(),
                "Write a main() that reads a sample file and prints its contents".to_string(),
            ]
        } else if desc.contains("write") && desc.contains("file") {
            vec![
                "Use std::fs::write or std::fs::File with BufWriter for file output".to_string(),
                "Prepare the content string to write".to_string(),
                "Handle the Result with proper error handling".to_string(),
                "Add comments explaining the file write process".to_string(),
                "Write a main() that writes sample content and confirms success".to_string(),
            ]
        } else if desc.contains("count") || desc.contains("frequency") {
            vec![
                "Define a function that takes a collection and returns a count or frequency map".to_string(),
                "Use a HashMap<K, usize> to track counts".to_string(),
                "Iterate through the input and increment counts".to_string(),
                "Add comments explaining the counting logic".to_string(),
                "Write a main() with a test input that prints the frequency results".to_string(),
            ]
        } else if desc.contains("filter") || desc.contains("remove") {
            vec![
                "Define a function that takes a Vec and a predicate closure".to_string(),
                "Use .filter() or .retain() to keep only elements matching the predicate".to_string(),
                "Return the filtered Vec".to_string(),
                "Add comments explaining the filter logic".to_string(),
                "Write a main() that demonstrates filtering with an example".to_string(),
            ]
        } else {
            // Generic steps using the task description words
            let noun = description
                .split_whitespace()
                .find(|w| w.len() > 4)
                .unwrap_or("task");
            vec![
                format!("Define the function signature for: {description}"),
                format!("Implement the core logic for {noun}"),
                "Handle edge cases and invalid inputs".to_string(),
                "Add comments explaining how the function works".to_string(),
                "Write a main() function with test cases and print the results".to_string(),
            ]
        }
    }
}
```

**Files affected:** `src/agents/planner.rs`

---

### Step 6: Rewrite `src/agents/coder.rs` — Pure Rust Coding Brain

The Coder detects task type and generates real, working Rust code — not a stub.

**New full content:**

```rust
use crate::messages::{CodePayload, PlanPayload};

/// The Coder agent writes Rust code based on the plan and task description.
/// Brain: Built-in — detects task type and generates working Rust code templates.
pub struct CoderAgent;

impl CoderAgent {
    pub fn new() -> Self { CoderAgent }

    pub fn process_with_task(&self, plan: PlanPayload, task_description: &str) -> CodePayload {
        println!("\n\x1b[1;34m[CODER]\x1b[0m Received plan with {} steps.", plan.steps.len());
        println!("\x1b[1;34m[CODER]\x1b[0m \x1b[2m· Brain: Built-in (Coding)\x1b[0m");
        println!("\x1b[1;34m[CODER]\x1b[0m Writing Rust code for: \"{}\"", task_description);

        let code = self.generate_code(task_description);

        println!("\x1b[1;34m[CODER]\x1b[0m Code written:");
        println!("\x1b[90m{}\x1b[0m", code);
        println!("\x1b[1;34m[CODER]\x1b[0m Handing off to Reviewer.");

        CodePayload {
            task_id: plan.task_id,
            code,
            language: "rust".to_string(),
        }
    }

    fn generate_code(&self, description: &str) -> String {
        let desc = description.to_lowercase();

        if desc.contains("sort") || desc.contains("order") {
            r#"/// Sorts a vector of integers in ascending order.
fn sort_numbers(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers
}

fn main() {
    let nums = vec![5, 2, 8, 1, 9, 3];
    println!("Before: {:?}", nums);
    let sorted = sort_numbers(nums);
    println!("After:  {:?}", sorted);
}"#.to_string()

        } else if desc.contains("revers") {
            r#"/// Reverses a string and returns the result.
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "hello world";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}"#.to_string()

        } else if desc.contains("fibonacci") || desc.contains("fib") {
            r#"/// Returns the nth Fibonacci number using an iterative approach.
fn fibonacci(n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

fn main() {
    println!("First 10 Fibonacci numbers:");
    for i in 0..10 {
        println!("  fib({}) = {}", i, fibonacci(i));
    }
}"#.to_string()

        } else if desc.contains("factorial") {
            r#"/// Computes n factorial iteratively.
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    for i in 0..=10 {
        println!("{}! = {}", i, factorial(i));
    }
}"#.to_string()

        } else if desc.contains("prime") {
            r#"/// Returns true if n is a prime number.
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn main() {
    println!("Primes up to 50:");
    let primes: Vec<u64> = (2..=50).filter(|&n| is_prime(n)).collect();
    println!("{:?}", primes);
}"#.to_string()

        } else if desc.contains("search") || desc.contains("find") {
            r#"/// Searches for a target value in a slice. Returns Some(index) or None.
fn linear_search(data: &[i32], target: i32) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

fn main() {
    let data = vec![3, 7, 1, 9, 4, 6, 2];
    let target = 9;
    match linear_search(&data, target) {
        Some(i) => println!("Found {} at index {}", target, i),
        None    => println!("{} not found", target),
    }
}"#.to_string()

        } else if desc.contains("count") || desc.contains("frequency") {
            r#"use std::collections::HashMap;

/// Counts how many times each element appears in the input slice.
fn count_frequency(items: &[&str]) -> HashMap<&str, usize> {
    let mut freq = HashMap::new();
    for &item in items {
        *freq.entry(item).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    let freq = count_frequency(&words);
    let mut pairs: Vec<_> = freq.iter().collect();
    pairs.sort_by_key(|&(k, _)| *k);
    for (word, count) in pairs {
        println!("{}: {}", word, count);
    }
}"#.to_string()

        } else if desc.contains("filter") || desc.contains("remove") {
            r#"/// Filters a vector, keeping only elements that satisfy the predicate.
fn filter_numbers(numbers: Vec<i32>, predicate: impl Fn(i32) -> bool) -> Vec<i32> {
    numbers.into_iter().filter(|&x| predicate(x)).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Keep only even numbers
    let evens = filter_numbers(numbers, |x| x % 2 == 0);
    println!("Even numbers: {:?}", evens);
}"#.to_string()

        } else {
            // Generic — derive a function name from the task description
            let func_name = description
                .to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() > 3)
                .take(2)
                .collect::<Vec<_>>()
                .join("_")
                .replace(|c: char| !c.is_alphanumeric() && c != '_', "");
            let func_name = if func_name.is_empty() { "run_task".to_string() } else { func_name };

            format!(
                "/// Implements: {description}\nfn {func_name}(input: &str) -> String {{\n    // TODO: implement logic for: {description}\n    format!(\"Result for: {{}}\", input)\n}}\n\nfn main() {{\n    let result = {func_name}(\"example input\");\n    println!(\"{{}}\", result);\n}}"
            )
        }
    }
}
```

**Files affected:** `src/agents/coder.rs`

---

### Step 7: Rewrite `src/agents/reviewer.rs` — Pure Rust Static Analysis Brain

**New full content:**

```rust
use crate::messages::{CodePayload, ReviewPayload};

/// The Reviewer agent checks code for quality issues using static analysis rules.
/// Brain: Built-in — applies Rust code quality rules directly.
pub struct ReviewerAgent;

impl ReviewerAgent {
    pub fn new() -> Self { ReviewerAgent }

    pub fn process(&self, code_payload: CodePayload) -> ReviewPayload {
        println!("\n\x1b[1;35m[REVIEWER]\x1b[0m Received code for review. Analyzing...");
        println!("\x1b[1;35m[REVIEWER]\x1b[0m \x1b[2m· Brain: Built-in (Security & Docs)\x1b[0m");

        let issues = self.review_code(&code_payload.code);
        let approved = issues.is_empty();

        if approved {
            println!("\x1b[1;35m[REVIEWER]\x1b[0m No issues found. Code approved.");
        } else {
            println!("\x1b[1;35m[REVIEWER]\x1b[0m Found {} issue(s):", issues.len());
            for issue in &issues {
                println!("\x1b[1;35m[REVIEWER]\x1b[0m   - {}", issue);
            }
        }
        println!("\x1b[1;35m[REVIEWER]\x1b[0m Handing off to Debugger.");

        ReviewPayload {
            task_id: code_payload.task_id,
            code: code_payload.code,
            issues,
            approved,
        }
    }

    fn review_code(&self, code: &str) -> Vec<String> {
        let mut issues = vec![];

        // Check 1: must have a main() function
        if !code.contains("fn main()") {
            issues.push("Missing fn main() — code needs an entry point".to_string());
        }

        // Check 2: should have at least one comment
        if !code.contains("//") && !code.contains("///") {
            issues.push("No comments found — add // or /// to explain the code".to_string());
        }

        // Check 3: excessive unwrap() usage is risky
        let unwrap_count = code.matches(".unwrap()").count();
        if unwrap_count > 3 {
            issues.push(format!(
                "Found {} .unwrap() calls — consider using match or ? for better error handling",
                unwrap_count
            ));
        }

        // Check 4: unimplemented placeholders left in code
        if code.contains("todo!()") || code.contains("unimplemented!()") {
            issues.push("Contains todo!() or unimplemented!() — finish the implementation".to_string());
        }

        // Check 5: empty function bodies
        if code.contains("{ }") || code.contains("{\n}") || code.contains("{\n    \n}") {
            issues.push("Found empty function body — implement the logic".to_string());
        }

        issues
    }
}
```

**Files affected:** `src/agents/reviewer.rs`

---

### Step 8: Rewrite `src/agents/debugger.rs` — Pure Rust Fix Pattern Brain

**New full content:**

```rust
use crate::messages::{FinalPayload, ReviewPayload};

/// The Debugger agent fixes issues identified by the Reviewer.
/// Brain: Built-in — applies known fix patterns to common Rust issues.
pub struct DebuggerAgent;

impl DebuggerAgent {
    pub fn new() -> Self { DebuggerAgent }

    pub fn process(&self, review: ReviewPayload) -> FinalPayload {
        println!("\n\x1b[1;31m[DEBUGGER]\x1b[0m \x1b[2m· Brain: Built-in (Debugging)\x1b[0m");

        if review.approved {
            println!("\x1b[1;31m[DEBUGGER]\x1b[0m Code approved. No fixes needed.");
        } else {
            println!("\x1b[1;31m[DEBUGGER]\x1b[0m Applying fixes for {} issue(s)...", review.issues.len());
        }

        let (fixed_code, summary) = self.fix(&review);

        println!("\x1b[1;31m[DEBUGGER]\x1b[0m {}", summary);
        println!("\x1b[1;31m[DEBUGGER]\x1b[0m Handing final code to Coordinator.");

        FinalPayload {
            task_id: review.task_id,
            code: fixed_code,
            summary,
        }
    }

    fn fix(&self, review: &ReviewPayload) -> (String, String) {
        if review.approved {
            return (review.code.clone(), "Code passed review — no changes needed.".to_string());
        }

        let mut code = review.code.clone();
        let mut fixes_applied = vec![];

        for issue in &review.issues {
            let issue_lower = issue.to_lowercase();

            // Fix: missing main()
            if issue_lower.contains("main()") {
                if !code.contains("fn main()") {
                    code.push_str("\n\nfn main() {\n    println!(\"Program complete.\");\n}");
                    fixes_applied.push("added fn main()");
                }
            }

            // Fix: missing comments
            if issue_lower.contains("comment") {
                if !code.contains("//") {
                    // Prepend a module-level comment
                    code = format!("// This module implements the requested functionality.\n\n{}", code);
                    fixes_applied.push("added missing comments");
                }
            }

            // Fix: todo!/unimplemented! placeholders
            if issue_lower.contains("todo") || issue_lower.contains("unimplemented") {
                code = code
                    .replace("todo!()", "String::new() // TODO: implement this")
                    .replace("unimplemented!()", "Default::default() // TODO: implement this");
                fixes_applied.push("replaced placeholders with stubs");
            }
        }

        let summary = if fixes_applied.is_empty() {
            format!("Reviewed {} issue(s) — no automatic fixes applied.", review.issues.len())
        } else {
            format!("Applied fixes: {}", fixes_applied.join(", "))
        };

        (code, summary)
    }
}
```

**Files affected:** `src/agents/debugger.rs`

---

### Step 9: Rewrite `src/agents/validator.rs` — Pure Rust Correctness Brain

**New full content:**

```rust
use crate::messages::{FinalPayload, ValidationPayload};

/// The Validator agent checks whether the code addresses the user's task.
/// Brain: Built-in — keyword and structure checks to confirm task match.
pub struct ValidatorAgent;

impl ValidatorAgent {
    pub fn new() -> Self { ValidatorAgent }

    pub fn process(&self, result: &FinalPayload, task_description: &str) -> ValidationPayload {
        println!("\n\x1b[1;33m[VALIDATOR]\x1b[0m Checking output matches task...");
        println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[2m· Brain: Built-in (Testing)\x1b[0m");

        let (passed, reason) = self.validate(&result.code, task_description);

        if passed {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[32mValidation passed.\x1b[0m");
        } else {
            println!("\x1b[1;33m[VALIDATOR]\x1b[0m \x1b[31mValidation failed: {reason}\x1b[0m");
        }

        ValidationPayload { task_id: result.task_id, passed, reason }
    }

    fn validate(&self, code: &str, task_description: &str) -> (bool, String) {
        let stop_words = [
            "a", "an", "the", "write", "create", "make", "build", "implement",
            "function", "in", "for", "that", "with", "and", "or", "to", "of",
        ];

        // Extract meaningful keywords from the task
        let keywords: Vec<String> = task_description
            .to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !stop_words.contains(w) && w.len() >= 3)
            .map(|w| w.to_string())
            .collect();

        let code_lower = code.to_lowercase();

        // Check 1: code must have a main() function
        if !code.contains("fn main()") {
            return (false, "Code is missing fn main() — no entry point".to_string());
        }

        // Check 2: at least one task keyword must appear in the code
        if keywords.is_empty() {
            return (true, "Task keywords too generic — basic structure check passed".to_string());
        }

        let matched = keywords.iter().any(|kw| code_lower.contains(kw.as_str()));

        if matched {
            (true, "Code contains task-relevant keywords and has a main() entry point".to_string())
        } else {
            (
                false,
                format!(
                    "Code does not appear to address the task — none of {:?} found in code",
                    &keywords[..keywords.len().min(3)]
                ),
            )
        }
    }
}
```

**Files affected:** `src/agents/validator.rs`

---

### Step 10: Update `src/agents/coordinator.rs` — Remove Stale Brain Label

**Actions:**
- Remove the line that prints `"· Brain: Claude (Architecture)"` — the Coordinator has no AI brain

**Files affected:** `src/agents/coordinator.rs`

---

### Step 11: Run `cargo build` — Verify Zero Errors

**Actions:**
- Run `cargo build`
- Most likely errors: any remaining `use crate::ai_client::...` imports, or `mod ai_client;` still in main.rs
- Fix all errors, then confirm clean build

**Files affected:** Build step only

---

### Step 12: Verify No Stale AI References

**Actions:**
- Search `src/` for: `ai_client`, `call_gpt`, `call_deepseek`, `call_claude`, `OPENAI`, `DEEPSEEK`, `ANTHROPIC`, `reqwest`, `serde_json`
- Remove any found

---

### Step 13: Update `CLAUDE.md`

**Actions:**
- Add `/refactor` command section to the Commands area
- Note that after running `/refactor` the project requires zero API keys

**Content to add (after /brain-chips section):**

```markdown
### /refactor

**Purpose:** Delete ALL external AI API code and API key requirements. Rewrites every agent with a specialized pure-Rust brain — no API keys, no external services.

When invoked, Claude will spawn 4 agents:
1. **Opus 4.5 (Architect)** — reads the codebase and plans the rewrite
2. **Sonnet 4.5 (Cleanup)** — deletes ai_client.rs, removes dependencies, cleans main.rs
3. **Sonnet 4.5 (Rewriter)** — rewrites all 5 agent files with built-in Rust brains
4. **Sonnet 4.5 (Validator)** — builds, verifies, updates docs, commits

After running: **zero API keys needed**. Just `cargo run`.

Built-in brain assignments:
- **Planner** — keyword-based task analysis → tailored implementation steps
- **Coder** — task-type detection → real working Rust code templates
- **Reviewer** — static analysis rules → code quality checks
- **Debugger** — known fix patterns → automatic issue resolution
- **Validator** — keyword + structure checks → task match confirmation
```

**Files affected:** `CLAUDE.md`

---

### Step 14: Append Feature Summary to Oldest Plan

**Actions:**
- Append to `plans/2026-02-17-rust-multi-agent-coding-system.md`:

```markdown

---

## Feature Added: Pure-Rust Agent Brains — Zero API Keys (2026-02-18)

**Command:** `/refactor`
**Status:** Implemented

### What Changed

- Deleted `src/ai_client.rs` entirely — no more external AI calls
- Removed `reqwest`, `serde`, `serde_json` from `Cargo.toml`
- Removed `mod ai_client;` from `src/main.rs`
- Rewrote all agents with specialized pure-Rust brains:
  - **Planner** — keyword analysis → task-type-specific step generation (10+ task types)
  - **Coder** — task-type detection → real working Rust code templates (sort, reverse, fibonacci, factorial, prime, search, count, filter, and more)
  - **Reviewer** — static analysis: checks for main(), comments, unwrap() overuse, placeholders, empty bodies
  - **Debugger** — applies fix patterns: adds main(), inserts comments, replaces placeholders
  - **Validator** — keyword + structure validation confirms code matches the task
- Zero API keys required — project runs fully standalone

### Why

Full removal of all external AI dependencies. Any user can clone and run immediately with just `cargo run`.
```

**Files affected:** `plans/2026-02-17-rust-multi-agent-coding-system.md`

---

### Step 15: Commit All Changes

**Actions:**
- `git add -A`
- `git commit -m "refactor: remove all AI APIs — agents now run on pure-Rust brains, zero API keys needed"`

---

## Connections & Dependencies

### Files That Reference This Area

- `src/pipeline.rs` — uses all agents; not modified, just depends on them compiling
- `src/main.rs` — entry point; only `mod ai_client;` line removed
- `.claude/commands/brain-chips.md` — historical, superseded by /refactor

### Updates Needed for Consistency

- `CLAUDE.md` updated with /refactor section
- Oldest plan updated with feature summary
- `Cargo.lock` will regenerate automatically when `cargo build` removes dependencies

---

## Validation Checklist

- [ ] `.claude/commands/refactor.md` exists
- [ ] `src/ai_client.rs` is deleted
- [ ] `Cargo.toml` has no `reqwest`, `serde`, `serde_json`
- [ ] `src/main.rs` has no `mod ai_client;`
- [ ] All agent files have no `use crate::ai_client::` imports
- [ ] `cargo build` succeeds with zero errors
- [ ] `grep -r "ai_client\|OPENAI\|DEEPSEEK\|ANTHROPIC\|call_gpt\|call_deepseek\|call_claude" src/` returns nothing
- [ ] `CLAUDE.md` documents `/refactor`
- [ ] Oldest plan has feature summary appended
- [ ] All changes committed

---

## Success Criteria

1. `cargo build` passes with zero errors
2. `grep -r "ai_client\|OPENAI\|DEEPSEEK\|ANTHROPIC" src/` returns nothing
3. `cargo run` launches and agents process a task using only built-in Rust logic
4. No API keys required at any point
5. All changes committed with message `refactor: remove all AI APIs — agents now run on pure-Rust brains, zero API keys needed`

---

## Notes

- `Cargo.lock` does not need to be manually edited — it regenerates when `cargo build` runs after `Cargo.toml` changes
- The `src/messages.rs`, `src/pipeline.rs`, `src/task.rs` files are not changed — the agent interfaces stay the same
- `src/agents/mod.rs` is not changed — all agent modules stay registered
- The Coordinator never had AI calls, so only its print label changes (remove the brain line)
