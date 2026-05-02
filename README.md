# 🔨 MACE (Multi-Agent Consensus Engine)

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](#license)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

> **A high-performance Meta-Agent orchestrator built in Rust, powered by Model-Consensus Driven Development (MCDD) and strict Compliance Governance.**

MACE is a command-line orchestration engine that does not write code directly. Instead, it commands, coordinates, and governs underlying AI programming tools (such as Claude Code, Gemini CLI, Aider, etc.). It enables high-concurrency code generation, physical sandbox isolation via Git Worktrees, DAG-based task routing, and automated code conflict resolution.

## 💡 The Problem MACE Solves

As AI coding assistants proliferate, single models often hit bottlenecks regarding context windows, logical reasoning, or API limitations. However, allowing multiple AI CLI tools to concurrently mutate the same repository leads to catastrophic race conditions, file overwrites, and LSP (Language Server Protocol) crashes.

MACE resolves this by acting as the ultimate arbiter:
1. **Breaking Monolithic Limits**: Dispatches specialized tasks to the most suitable models (e.g., Claude for core logic, Gemini for massive context analysis).
2. **Fearless Concurrency**: Pioneers the use of **Git Worktree** to provision isolated, physical directory sandboxes for each sub-agent, enabling true parallel execution on the same codebase without collisions.
3. **Semantic Consensus (MCDD)**: Moves beyond noisy text-based `git diffs`. MACE utilizes AST (Abstract Syntax Tree) parsing to filter out trivial formatting differences, triggering the Model-Consensus engine only for true logical or interface conflicts.
4. **Audit & Compliance Governance**: Every line of AI-generated code is tracked, debated, and locked, ensuring complete traceability and responsibility attribution for enterprise environments.

## 🏗️ Architecture & Workflow

A typical MACE task lifecycle operates as a strict Finite State Machine (FSM):

1. **DAG Topology Parsing**: Parses the user's prompt and breaks it down into a Directed Acyclic Graph (DAG) of atomic instructions, defining dependency chains between different agent roles.
2. **Workspace Provisioning & Blackboard Integration**: Utilizes `git worktree add` to automatically allocate isolated physical directories. Concurrently, a `.mace_blackboard` (read-only shared context containing API specs/schemas) is mounted to all worktrees.
3. **Subprocess Orchestration**: Uses Rust's `tokio::process` to spawn headless AI CLI subprocesses, hijacking `stdio` to monitor execution states and suppress redundant UI outputs.
4. **Strict Consensus Gate & Merge**: MACE does not consider a task complete just because subprocesses exit. It extracts diffs, runs Tree-sitter AST validation, and forces agents into a **Consensus Loop**:
   - If diffs align semantically, `git merge` is executed.
   - If divergent, the MCDD engine triggers a debate (e.g., `coder` justifying changes to the `architect`).
   - **Deadlock Resolution (HITL)**: If sub-agents fail to reach consensus after $N$ rounds, MACE pauses orchestration, preserves sandboxes, and requests Human-in-the-Loop intervention.
5. **Teardown & Audit Logging**: Upon successful consensus, sandboxes are safely garbage-collected, and an immutable `.mace.lock` audit trail is generated.

## ⚙️ Prerequisites

MACE acts exclusively as the **Orchestrator**. It does not bundle LLM API calls or manage API keys directly. Before running MACE, you must independently install and authenticate the underlying AI CLI tools on your local machine or compute node.

Currently supported and tested local agents:
- `claude-code` (Anthropic)
- `gemini-cli` (Google)
- `aider` (Multi-model)
- `copilot-cli` (GitHub)
- *Custom binaries (Configurable via `.mace.toml`)*

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

# Dispatch a concurrent task using the defined roles
mace run "Refactor the Auth module to use stateless JWTs" --roles="architect,coder,reviewer"

# Audit mode: Force MACE to cross-review the current git staging area
mace audit --strategy="cross-review"

# Resume an interrupted task or HITL deadlock from the exact FSM state
mace resume <task_id>
```

### Role-Based Orchestration

MACE allows you to define specialized roles in your `.mace.toml` to enforce "Separation of Concerns" among AI agents:
```toml
# .mace.toml example
[roles]
architect = { agent = "claude-code", model = "claude-3.7-sonnet", prompt_template = "architect.md" }
coder = { agent = "aider", model = "deepseek-coder", prompt_template = "implementer.md" }
reviewer = { agent = "gemini-cli", model = "gemini-2.5-pro", context_window = "large" }

[consensus]
strategy = "majority-vote" # Options: cross-review, majority-vote, referee-arbitration
referee = "architect"      # The role that makes the final call in a deadlock
max_rounds = 3             # Triggers HITL (Human-in-the-loop) if exceeded
semantic_diff = true       # Enable AST-level conflict filtering
```

## 🛠️ Technology Stack

- **Core Engine**: Rust (Tokio for async runtime)
- **TUI Framework**: Ratatui & Crossterm (for multi-pane execution monitoring)
- **Subprocess Management**: Native `std::process` / `tokio::process`
- **VCS Integration**: Deeply integrated with `git`, specifically `git worktree` and underlying merge commands.
- **Semantic Engine**: Tree-sitter (for AST parsing and noise reduction)

---

### ⚠️ System Prompt for AI Coding Assistants
*If you are an AI assistant (like Cursor, Claude, or Copilot) reading this repository to help develop MACE, you MUST adhere to the following rules:*

1. **Do Not Implement LLM API Calls**: MACE does not communicate directly with OpenAI or Anthropic APIs to generate code. Its primary job is to **spawn and govern other CLI binaries**.
2. **State Management is Paramount**: When manipulating Git Worktrees, ensure that temporary directories and hidden branches are safely garbage-collected regardless of whether the subprocess succeeds, panics, or is killed via `SIGINT`.
3. **Concurrency Safety**: Leverage Rust's ownership model and `tokio` channels strictly. Avoid blocking the Ratatui UI thread when reading from subprocess stdout pipes.

---

## 🗺️ Roadmap

- [ ] **Phase 1 (Observer)**: Cross-CLI spawning, stdout hijacking, and Ratatui multi-pane monitoring.
- [ ] **Phase 2 (Executor)**: Full lifecycle management of Git Worktrees (provision, physical isolation, teardown) and FSM state recovery.
- [ ] **Phase 3 (Consensus)**: Delivery of the MCDD consensus layer (Cross-Review, Referee Arbitration) and AST-based semantic diff filtering.
- [ ] **Phase 4 (Governance)**: Implementation of DAG task routing, the Blackboard shared-context pattern, and `.mace.lock` audit trails.

## 📄 License

This project is licensed under the **Apache License, Version 2.0**.

Copyright (c) 2026 Janber Zhang. All rights reserved.

See the [LICENSE](LICENSE) file for explicit details.