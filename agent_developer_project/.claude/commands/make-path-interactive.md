# Make Path Interactive

Rebuild the agent-team release binary and reinstall it to `~/.local/bin/` so that
running `agent-team` from anywhere in the terminal launches interactive mode by default —
the same as running `cargo run` from the project directory.

---

## Instructions

### Phase 1: Rebuild the Release Binary

Run the install script which rebuilds and reinstalls in one step:

```bash
bash scripts/install.sh
```

If the script succeeds, the binary at `~/.local/bin/agent-team` is now up to date.
If `cargo build --release` fails, read the error output and fix the compilation error before continuing.

---

### Phase 2: Verify Interactive Mode Is Active

Run the installed binary directly by path to confirm it launches interactive mode:

```bash
echo "exit" | ~/.local/bin/agent-team
```

The output should contain the text `INTERACTIVE MODE ACTIVE`.

If `~/.local/bin` is in PATH, also test:

```bash
echo "exit" | agent-team
```

---

### Phase 3: Report

After completion, tell the user:

1. **Build result:** whether `cargo build --release` succeeded
2. **Install location:** `~/.local/bin/agent-team`
3. **Verification:** whether the binary printed `INTERACTIVE MODE ACTIVE`
4. **How to use:** run `agent-team` from any terminal — it will launch in interactive mode
