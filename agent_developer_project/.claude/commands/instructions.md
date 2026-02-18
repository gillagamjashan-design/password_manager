# Instructions

> Give the user clear, beginner-friendly instructions for how to install and run this project.

## Read

- `README.md`
- `Cargo.toml`
- `scripts/install.sh` (if it exists)

## Instructions to Present

After reading the files above, present the following to the user in a clean, well-formatted way:

### 1. Prerequisites

Explain that Rust must be installed first. If the user doesn't have it:

- Go to https://rustup.rs
- Follow the instructions for their operating system
- Verify with: `rustc --version`

### 2. Get the Project

Tell the user to clone or download the project, then open a terminal in the project folder.

### 3. Build the Project

```bash
cargo build
```

Explain this compiles the Rust source code. Only needs to be done once (or after making code changes).

### 4. Run the Project

```bash
cargo run
```

Explain this runs the agent team. They will see all 5 agents working through 3 sample tasks, printing their progress to the terminal.

### 5. Install to PATH (Optional — Run from Anywhere)

If `scripts/install.sh` exists, tell the user they can install the binary system-wide:

```bash
bash scripts/install.sh
source ~/.bashrc
```

Then they can run from any directory:

```bash
agent-team
```

### 6. Adding Custom Tasks

Show the user how to add their own tasks in `src/main.rs`:

```rust
pipeline.run("write a function that checks if a string is a palindrome");
```

## Tone

- Be friendly and encouraging
- Use numbered steps so it's easy to follow
- Avoid technical jargon unless necessary — this project is for beginners
- Remind the user they can ask questions if anything is unclear
