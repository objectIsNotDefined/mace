# 🤖 MACE: AI Core Rules & Guidelines

**IMPORTANT**: All AI coding assistants (Cursor, Claude, Copilot, Aider, etc.) MUST adhere to these rules when working on the MACE project.

## 1. Project Identity & Architecture
MACE (Multi-Agent Consensus Engine) is an enterprise-grade **Meta-Agent Orchestrator** built in Rust. 
*   **DO NOT implement LLM API calls directly** (e.g., no OpenAI/Anthropic SDKs). MACE's job is to spawn, manage, and arbitrate *other* AI CLIs.
*   **FSM Lifecycle**: Task execution follows a strict Finite State Machine defined in the roadmap.
*   **Physical Isolation**: Concurrency is handled via **Git Worktrees** and invisible temporary branches, not in-memory diffs.

## 2. Rust Coding Standards
*   **Async Runtime**: Use `tokio`. Never block the async runtime with heavy synchronous tasks; use `tokio::task::spawn_blocking` if necessary.
*   **Error Handling**: Use `anyhow` or `thiserror`. **DO NOT** use `.unwrap()` or `.expect()` in production code. Always propagate errors safely.
*   **TUI Framework**: Use `ratatui` with `crossterm`. The UI thread must remain unblocked. Use `tokio::sync::mpsc` channels to stream logs and state changes from child processes to the UI.
*   **Subprocesses**: Use `tokio::process::Command` to spawn AI CLIs.

## 3. Concurrency & State Safety
*   **Garbage Collection**: MACE creates temporary Git Worktrees. Ensure they are safely cleaned up. Implement `Drop` traits or global signal handlers (`tokio::signal::ctrl_c`) to prevent codebase pollution upon `Panic` or `SIGINT`.
*   **I/O Hijacking**: When reading `stdout`/`stderr` from child processes, be prepared to handle and strip ANSI escape codes and PTY drawing characters (e.g., using `strip-ansi-escapes`).

## 4. MCDD (Model-Consensus Driven Development)
*   **Semantic Diffing**: Do not rely on simple text diffs. Use `tree-sitter` to parse ASTs and filter out whitespace/formatting noise before declaring a conflict.
*   **Read the Roadmap**: Refer to `roadmap/roadmap-phaseX.md` for specific implementation details before writing features.
