# Phase 2: Executor (Physical Sandbox & Workflow Engine) Design Document

## 🎯 Core Objectives
Resolve race conditions and file overwrite issues when multiple AI tools concurrently modify the same codebase. Implement physical-level sandbox isolation by introducing Git Worktree, and build an FSM (Finite State Machine) workflow engine to provide task fault tolerance and recovery capabilities.

## 🏗️ Architecture Design & Technology Stack

### 1. Git Worktree Sandbox Manager (`sandbox`)
*   **Responsibility**: Automatically allocate and destroy fully isolated code working directories for each concurrent Agent.
*   **Dependencies (Crates)**: `git2` (native libgit2 bindings) or wrap git CLI operations directly via `std::process::Command`. The latter is recommended as the Worktree API might be incomplete and overhead-heavy in `git2-rs`.
*   **Isolation Strategy**:
    *   Create physical sandboxes within the `.mace/worktrees/` directory.
    *   Create a hidden temporary branch based on the current branch (e.g., `.mace-branch-<role>-<task_id>`).
    *   Mount the sandbox using `git worktree add <path> <branch>`.

### 2. Finite State Machine Core Engine (`fsm`)
*   **Responsibility**: Orchestrate the complete lifecycle of tasks, ensuring atomicity and recoverability of operations.
*   **State Definitions**:
    1.  `Init`: Parse tasks and pre-check the environment.
    2.  `Provisioning`: Allocate all Git Worktree sandboxes.
    3.  `Executing`: Wake up all AI child processes to execute tasks.
    4.  `Consensus_Pending`: Processes finish, waiting to extract Diffs.
    5.  `Resolving`: If divergence occurs, enter multi-round arbitration logic.
    6.  `Merge_Success`: Final merge into the main workspace.
    7.  `Teardown`: Destroy sandboxes and branches, enter the completion state.

### 3. State Persistence & Recovery Mechanism (`recovery`)
*   **Mechanism**: Serialize and save the current state context to `.mace/state/<task_id>.json` during every FSM state transition.
*   **Recovery Entry**: Provide the CLI command `mace resume <task_id>`. The system reads the JSON and resumes the workflow directly from the breakpoint state (like `Executing` or `Resolving`), avoiding wasting token costs by starting over.

### 4. Mandatory Garbage Collection (`gc`)
*   **Responsibility**: Ensure temporary files and branches do not pollute the user's codebase.
*   **Implementation Means**:
    *   Implement Rust's `Drop` trait bound to the sandbox manager.
    *   Register process signal listeners (`tokio::signal::ctrl_c`). Upon catching an interrupt signal, forcibly trigger the cleanup process: execute `git worktree remove --force` and `git branch -D`.

## 📝 Detailed Task Breakdown & Status Tracking

- [ ] **1. Worktree Management Module Development**
  - [ ] 1.1 Wrap Rust APIs for `git worktree add/list/remove`.
  - [ ] 1.2 Write logic for automatic sandbox addressing and naming generation.
  - [ ] 1.3 Write unit tests to verify isolated sandbox environments do not affect each other.
- [ ] **2. FSM Workflow Engine Implementation**
  - [ ] 2.1 Define the Enum state enumeration for the task lifecycle.
  - [ ] 2.2 Implement the core `tick` engine for state transitions.
  - [ ] 2.3 Embed Phase 1's Agent Runner into the `Executing` state.
- [ ] **3. State Persistence Storage**
  - [ ] 3.1 Define the Serde serialization structure for the state context.
  - [ ] 3.2 Implement atomic writes for `.mace_state.json`.
  - [ ] 3.3 Implement `mace resume` loading logic.
- [ ] **4. Disaster Recovery & Cleanup Mechanism Refinement**
  - [ ] 4.1 Write `SignalHandler` to catch `Ctrl+C` and `SIGTERM`.
  - [ ] 4.2 Implement `Drop` cleanup logic to handle safe exits when processes crash (Panic).

---
**Current Phase Status**: ⏳ Not Started
