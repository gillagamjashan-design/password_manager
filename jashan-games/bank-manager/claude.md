# Claude Code Guidelines for Bank Manager Project

## Project Context
This is a learning/experimental game development project. The focus is on exploration, testing ideas, and building skills.

## Commit Policy
- **Always commit changes** after completing tasks or making significant modifications
- **Auto-commit mode**: Commit automatically after task completion without asking for permission
- **Commit message style**: Use professional, clear commit messages that describe what was changed and why
  - Format: Start with an action verb (Add, Update, Fix, Remove, Refactor, etc.)
  - Be specific about what changed
  - Example: "Add player inventory system with item management"
  - Example: "Fix memory leak in game loop rendering"

## File Management & .gitignore
Before committing any changes, ensure unnecessary files are added to .gitignore:

### Always ignore:
- Build artifacts: `dist/`, `build/`, `out/`, `*.o`, `*.exe`, `a.out`
- Logs and temporary files: `*.log`, `*.tmp`, `temp/`, `tmp/`
- IDE/editor files: `.vscode/`, `.idea/`, `*.swp`, `*.sublime-*`, `.DS_Store`
- Dependencies: `node_modules/`, `vendor/`, `__pycache__/`, `*.pyc`
- System files: `.gemini/`, `Thumbs.db`
- Database files (unless critical): `*.db` (evaluate case-by-case)

### Keep in repository:
- Source code files
- Configuration files needed for the project to run
- Documentation
- Essential assets (sprites, sounds, etc.)
- Build scripts and project files

## Workflow
1. Read and understand existing code before making changes
2. Make requested changes
3. Clean up unnecessary files → add to .gitignore if needed
4. Commit changes with professional message
5. Keep changes focused and atomic when possible

## Questions to Consider
When working on this project, consider asking:
- Is this the right approach for a learning/experimental project?
- Are there simpler ways to implement this feature?
- Should we explore alternative solutions?
- What would you like to learn from this implementation?

---

*This file guides Claude Code's behavior in this repository. Modify as needed for your workflow.*
