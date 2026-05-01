# 🔨 MACE (Multi-Agent Consensus Engine)

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](#license)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

> **A high-performance Meta-Agent orchestrator built in Rust, powered by Model-Consensus Driven Development (MCDD).**

MACE is a command-line orchestration engine that does not write code directly. Instead, it commands, coordinates, and governs underlying AI programming tools (such as Claude Code, Gemini CLI, Aider, etc.). It enables high-concurrency code generation, physical sandbox isolation via Git Worktrees, and automated code conflict resolution.

## 💡 The Problem MACE Solves

As AI coding assistants proliferate, single models often hit bottlenecks regarding context windows, logical reasoning, or API limitations. However, allowing multiple AI CLI tools to concurrently mutate the same repository leads to catastrophic race conditions, file overwrites, and LSP (Language Server Protocol) crashes.

MACE resolves this by acting as the ultimate arbiter:
1. **Breaking Monolithic Limits**: Dispatches specialized tasks to the most suitable models (e.g., Claude for core logic, Gemini for massive context analysis).
2. **Fearless Concurrency**: Pioneers the use of **Git Worktree** to provision isolated, physical directory sandboxes for each sub-agent, enabling true parallel execution on the same codebase without collisions.
3. **MCDD Consensus Protocol**: When sub-agents produce divergent code, MACE captures the diffs and triggers a consensus algorithm (Cross-Review, Majority Vote, or Referee Arbitration) to determine the optimal merge strategy.

## 🏗️ Architecture & Workflow

A typical MACE task lifecycle:

1. **Task Decomposition**: Parses the user's prompt and breaks it down into atomic execution instructions.
2. **Workspace Provisioning**: Utilizes `git worktree add` to automatically allocate isolated physical directories and invisible branches (e.g., `mace-task-claude`) for parallel execution.
3. **Subprocess Orchestration**: Uses Rust's `tokio::process` to spawn headless AI CLI subprocesses, hijacking `stdio` to monitor execution states and suppress redundant UI outputs.
4. **Consensus & Merge**: Upon task completion, MACE reclaims the worktrees and executes `git merge` on the main branch. If conflicts arise, the MCDD engine takes over to resolve them.

## 🚀 Quick Start

### Installation

Ensure you have Rust and Cargo installed, then run:

```bash
cargo install mace
```

### Basic Usage

```bash
# Initialize MACE configuration (.mace.toml) in your current repository
mace init

# Dispatch a concurrent task to multiple agents
mace run "Refactor the Auth module to use stateless JWTs" --models="claude-code,gemini-cli"

# Audit mode: Force MACE to cross-review the current git staging area
mace audit --strategy="cross-review"
```

## 🛠️ Technology Stack

- **Core Engine**: Rust (Tokio for async runtime)
- **TUI Framework**: Ratatui & Crossterm (for multi-pane execution monitoring)
- **Subprocess Management**: Native `std::process` / `tokio::process`
- **VCS Integration**: Deeply integrated with `git`, specifically `git worktree` and underlying merge commands.

---

### ⚠️ System Prompt for AI Coding Assistants
*If you are an AI assistant (like Cursor, Claude, or Copilot) reading this repository to help develop MACE, you MUST adhere to the following rules:*

1. **Do Not Implement LLM API Calls**: MACE does not communicate directly with OpenAI or Anthropic APIs to generate code. Its primary job is to **spawn and govern other CLI binaries**.
2. **State Management is Paramount**: When manipulating Git Worktrees, ensure that temporary directories and hidden branches are safely garbage-collected regardless of whether the subprocess succeeds, panics, or is killed via `SIGINT`.
3. **Concurrency Safety**: Leverage Rust's ownership model and `tokio` channels strictly. Avoid blocking the Ratatui UI thread when reading from subprocess stdout pipes.

---

## 🗺️ Roadmap

- [ ] **Phase 1 (Observer)**: Implement cross-CLI spawning, stdout hijacking, and basic Ratatui multi-pane monitoring.
- [ ] **Phase 2 (Executor)**: Full lifecycle management of Git Worktrees (provision, execute, diff extraction, teardown).
- [ ] **Phase 3 (Autonomous)**: Deliver the complete MCDD consensus layer for automated AI diff arbitration.

## 📄 License

This project is licensed under the **Apache License, Version 2.0**.

Copyright (c) 2026 Janber Zhang. All rights reserved.

See the [LICENSE](LICENSE) file for explicit details.