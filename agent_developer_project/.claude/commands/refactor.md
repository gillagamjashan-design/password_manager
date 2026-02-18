# Refactor

Delete ALL external AI API code and rewrite every agent with a specialized pure-Rust brain. Zero API keys needed after this runs.

## What This Does

1. Deletes src/ai_client.rs if it exists
2. Removes reqwest, serde, serde_json from Cargo.toml if present
3. Removes `mod ai_client` from src/main.rs if present
4. Rewrites all 5 agent files so each has its own specialized pure-Rust brain
5. Verifies the project compiles
6. Updates CLAUDE.md with /refactor command section
7. Commits all changes
8. Reports done — zero API keys needed, just `cargo run`

## Agent Brain Assignments After Refactor

| Agent | Brain | Specialty |
|---|---|---|
| Coordinator | Built-in | Task routing and final output presentation |
| Planner | Built-in | Keyword-based task decomposition → tailored steps |
| Coder | Built-in | Task-type detection → working Rust code templates |
| Reviewer | Built-in | Static analysis rules → code quality issues |
| Debugger | Built-in | Known fix patterns → automatic issue resolution |
| Validator | Built-in | Keyword + structure checks → task match confirmation |

## Run

Use the Task tool to spawn 4 agents in sequence: Agent 1 runs first, then Agents 2 and 3 run in parallel, then Agent 4 runs last.

**IMPORTANT:** Each agent uses the Task tool with subagent_type="general-purpose" and the specified model. Pass all context in the prompt — subagents cannot see the parent conversation.

---

### Agent 1 (Opus 4.5) — Architect

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"` and `model="opus"`.

**Prompt to pass:**

```
You are the Architect agent for a Rust multi-agent coding project refactor.

Your job: Read all source files in /workspace/jashan/agent_developer_project/src/ and plan what each agent's specialized pure-Rust brain will do. Do NOT write any code yet — only research and plan.

Read these files:
- src/main.rs
- src/agents/coordinator.rs
- src/agents/planner.rs
- src/agents/coder.rs
- src/agents/reviewer.rs
- src/agents/debugger.rs
- src/agents/validator.rs
- Cargo.toml

For each agent file, note:
1. Does it import or use any external AI API (reqwest, serde_json, ai_client)?
2. What is its current brain logic?
3. What specialized pure-Rust brain would be best for its role?

Return a detailed plan describing:
- Which files need cleanup (API removal)
- What each agent's pure-Rust brain should do, with specific implementation ideas
- Any Cargo.toml dependencies to remove

Format as a clear numbered plan that Agents 2, 3, and 4 can follow.
```

Wait for Agent 1 to return its plan before proceeding.

---

### Agent 2 (Sonnet 4.5) — Cleanup

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"` and `model="sonnet"`.

**Prompt to pass (include Agent 1's plan output):**

```
You are the Cleanup agent for a Rust multi-agent coding project refactor.

Working directory: /workspace/jashan/agent_developer_project

Your job: Remove all external AI API code from the project. Here is the Architect's plan:

[INSERT AGENT 1 OUTPUT HERE]

Execute these cleanup steps:
1. Check if src/ai_client.rs exists — if so, delete it
2. Read src/main.rs — remove any `mod ai_client;` line if present. Save the file.
3. Read Cargo.toml — remove reqwest, serde, serde_json from [dependencies] if present. Save the file.
4. Run `cargo check` from /workspace/jashan/agent_developer_project to confirm no compilation errors after cleanup.

Report exactly what you changed and whether cargo check passed.
```

---

### Agent 3 (Sonnet 4.5) — Rewrite Agents

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"` and `model="sonnet"`.

**Prompt to pass (include Agent 1's plan output):**

```
You are the Rewrite agent for a Rust multi-agent coding project refactor.

Working directory: /workspace/jashan/agent_developer_project

Your job: Rewrite all agent files so each has a specialized pure-Rust brain — no external AI APIs, no HTTP calls, just smart built-in Rust logic. Here is the Architect's plan:

[INSERT AGENT 1 OUTPUT HERE]

For each agent file, keep the same struct name, public methods, and message types. Only replace or improve the internal brain logic. Each agent's brain must be specialized for its role:

**planner.rs** — Keyword-based task decomposition:
- Detect task type from description keywords (sort, reverse, fibonacci, factorial, prime, search, find, count, filter, palindrome, anagram, stack, queue, hash, tree, graph, etc.)
- Return a tailored list of implementation steps specific to that task type
- Cover at least 12 distinct task types with their own step lists
- For unknown tasks: derive steps from the description intelligently

**coder.rs** — Task-type code generation:
- Detect task type from description keywords (same list as planner)
- Return real, working, runnable Rust code for each task type
- Code must compile and produce meaningful output
- Cover the same 12+ task types as planner
- For unknown tasks: generate a sensible stub with a real function signature

**reviewer.rs** — Static analysis:
- Check for: missing fn main(), missing comments (// or ///), excessive .unwrap() calls (>3), todo!() or unimplemented!() placeholders, magic numbers without explanation
- Return a list of specific issues found
- Approve code that passes all checks

**debugger.rs** — Issue resolution:
- Apply automatic fixes for each known issue type
- Add fn main() if missing
- Add a top-level comment if comments are absent
- Replace todo!() / unimplemented!() with sensible defaults
- Report exactly what was fixed

**validator.rs** — Task match confirmation:
- Extract meaningful keywords from the task description (filter stop words)
- Check that the code contains relevant keywords or function names
- Verify fn main() exists
- Return passed=true with reason, or passed=false with specific failure reason

**coordinator.rs** — Only update if it has stale labels or AI references. Keep its routing and display logic intact.

After rewriting each file, run `cargo build` from /workspace/jashan/agent_developer_project. If it fails, fix the errors and rebuild.

Report: which files you changed, what the new brain logic is for each, and whether the build succeeded.
```

Run Agents 2 and 3 in parallel (both can start after Agent 1 finishes, since they work on different parts: Agent 2 on cleanup, Agent 3 on agent rewrites).

---

### Agent 4 (Sonnet 4.5) — Validate and Finalize

**How to spawn:** Use the Task tool with `subagent_type="general-purpose"` and `model="sonnet"`. Run this AFTER Agents 2 and 3 complete.

**Prompt to pass:**

```
You are the Finalize agent for a Rust multi-agent coding project refactor.

Working directory: /workspace/jashan/agent_developer_project

Your job: Verify the refactor is complete, then commit and report done.

Steps:
1. Run `cargo build` from /workspace/jashan/agent_developer_project — must succeed with zero errors. If it fails, read the error, fix it, and try again.
2. Run `grep -r "ai_client\|reqwest\|OPENAI_API_KEY\|ANTHROPIC_API_KEY\|DEEPSEEK_API_KEY" src/` — confirm no AI API references remain. If any are found, remove them.
3. Run `cargo run` and send it "fibonacci" as stdin, then "exit". Capture and display the output.
4. Git commit all changes: `git -C /workspace/jashan/agent_developer_project add -A && git -C /workspace/jashan/agent_developer_project commit -m "refactor: give each agent a specialized pure-Rust brain — zero API keys needed"`
5. Report: build status, any remaining AI references, sample output from cargo run, and confirm the commit was made.
```

---

## After All 4 Agents Finish

Report to the user:
- Which files were changed
- What each agent's new brain does
- That `cargo run` works with zero API keys
- The git commit hash

Done.
