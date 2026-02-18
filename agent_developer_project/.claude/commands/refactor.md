# Refactor

Delete ALL external AI API code and API key requirements. Rewrite every agent with a specialized pure-Rust brain. Zero API keys needed after this runs.

## What This Does

1. Deletes src/ai_client.rs entirely
2. Removes reqwest, serde, serde_json from Cargo.toml
3. Removes mod ai_client from src/main.rs
4. Rewrites all 5 AI-using agents with smart pure-Rust brains
5. Verifies the project compiles
6. Updates CLAUDE.md and appends feature summary to the oldest plan
7. Commits all changes
8. Reports done — zero API keys needed, just cargo run

## No Setup Required After This

Zero API keys. Zero external services. Just: cargo run

## Agent Assignments After Refactor

| Agent | Brain | Specialty |
|---|---|---|
| Coordinator | Built-in | Task management |
| Planner | Built-in | Keyword-based step generation |
| Coder | Built-in | Task-type code templates |
| Reviewer | Built-in | Static analysis rules |
| Debugger | Built-in | Known fix patterns |
| Validator | Built-in | Keyword + structure checks |

## Run

Spawn 4 Claude agents:

### Agent 1 (Opus 4.5) — Architect
Read all files in src/ and src/agents/. Inventory every AI import and function call across the codebase. Plan what each agent's pure-Rust brain will do. Review Agent 3 work when done.

### Agent 2 (Sonnet 4.5) — Cleanup
1. Delete src/ai_client.rs
2. Remove mod ai_client from src/main.rs
3. Remove reqwest, serde, serde_json from Cargo.toml

### Agent 3 (Sonnet 4.5) — Rewrite Agents
Rewrite all 6 agent files — pure Rust brains, no external AI imports:
- planner.rs: keyword analysis generates task-specific steps (sort, reverse, fibonacci, factorial, prime, search, count, filter)
- coder.rs: task-type detection generates working Rust code templates for each task type
- reviewer.rs: static analysis checks (missing main, missing comments, excessive unwrap, placeholders)
- debugger.rs: applies fix patterns for each issue type (add main, add comments, replace placeholders)
- validator.rs: keyword + structure checks confirm code matches the task
- coordinator.rs: remove stale brain label

### Agent 4 (Sonnet 4.5) — Validate and Finalize
After agents 2 and 3:
- Run cargo build — must succeed zero errors, fix if not
- Confirm no AI references remain in src/
- Update CLAUDE.md with /refactor command section
- Append feature summary to oldest plan
- Commit all changes
- Report done
