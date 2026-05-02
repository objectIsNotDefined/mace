# Copilot Instructions for MACE

When generating code for this repository, please adhere to the project's core guidelines:

1. **Read `AI_RULES.md`**: This file contains the mandatory coding standards, architectural patterns, and error-handling requirements for MACE.
2. **Context**: MACE is a Rust-based orchestrator using `tokio`, `ratatui`, and `git worktree`.
3. **Strict Rust**: Avoid `.unwrap()`, propagate errors via `anyhow`, and ensure unblocked async execution.

See `.llms.txt` for an overview of the project structure.
