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
