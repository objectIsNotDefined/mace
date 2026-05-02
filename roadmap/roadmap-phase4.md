# Phase 4: Governance (Enterprise Governance & DAG Routing) Design Document

## 🎯 Core Objectives
Elevate MACE to an enterprise-grade workflow engine supporting complex Directed Acyclic Graph (DAG) task dependency decomposition. Introduce the "Blackboard Pattern" to resolve context consistency issues among multiple Agents, and generate tamper-proof `.mace.lock` files to achieve complete compliance traceability.

## 🏗️ Architecture Design & Technology Stack

### 1. DAG Task Routing Engine (`router`)
*   **Responsibility**: Parse the user's macro prompts and break them down into a series of sequentially dependent subtasks (e.g., output architectural design first, then write backend and frontend in parallel, and finally do code review).
*   **Dependencies (Crates)**: `petgraph` (Graph data structures and algorithms library).
*   **Mechanism**:
    *   Use an LLM call for the initial Task Planning, generating step instructions in JSON/YAML format.
    *   Deserialize the steps into a `petgraph::Graph`.
    *   Dispatch subtasks sequentially via topological sorting; parallelizable nodes are distributed to different Worktrees for concurrent execution.

### 2. Blackboard Shared Context Pattern (`blackboard`)
*   **Responsibility**: Ensure concurrent Agents possess a unified, up-to-date system-level context (like database Schema changes, interface definition changes).
*   **Architecture Design**:
    *   Create an implicit `.mace_blackboard/` directory to store domain knowledge, API documents, etc.
    *   This directory will be mounted (or symlinked) into all active Worktree sandboxes.
    *   **Event Bus**: If an Architect Agent modifies a blackboard file, MACE captures the file modification event and immediately broadcasts it to other Agent processes to reload the context.

### 3. Compliance Audit Trail Log (`audit`)
*   **Responsibility**: Provide traceability proof and accountability basis for every line of AI-generated code in enterprise environments.
*   **Log Design (`.mace.lock`)**:
    *   Before executing the final `git merge`, generate a lock file containing timestamps, task descriptions, models involved, AST review results, and the final conflict resolution method.
    *   Add special Git Signatures or Footer comments (like `Co-authored-by: Claude-3.7-sonnet via MACE`) to the final merged commit.

### 4. One-Click Version Control Linkage (`vcs`)
*   **Responsibility**: Safely merge the final code into the workspace after consensus is reached, and provide rollback capabilities based on the audit log.
*   **Mechanism**:
    *   Generate clear, atomic commits.
    *   Implement `mace rollback <audit_id>`, reading `.mace.lock` records to quickly reverse previous AI modifications.

## 📝 Detailed Task Breakdown & Status Tracking

- [ ] **1. Task Decomposition & DAG Scheduler**
  - [ ] 1.1 Develop the `TaskPlanner` module to decompose user natural language Prompts into execution trees.
  - [ ] 1.2 Integrate `petgraph` to implement graph node dependency execution and topological sort traversal.
- [ ] **2. Blackboard Context Synchronization**
  - [ ] 2.1 Design internal file formats and storage rules for `.mace_blackboard`.
  - [ ] 2.2 Integrate file system monitoring (`notify` crate) to implement a "Modify -> Interrupt -> Hot Reload" workflow.
- [ ] **3. Audit Logging System Construction**
  - [ ] 3.1 Design the Schema structure of `.mace.lock` (JSON or TOML).
  - [ ] 3.2 Implement an audit information collector spanning the entire task lifecycle.
- [ ] **4. Git Commit Integration & Rollback Tools**
  - [ ] 4.1 Automatically generate structured Git Commit Messages containing MACE audit information.
  - [ ] 4.2 Develop the CLI subcommand `mace rollback`.

---
**Current Phase Status**: ⏳ Not Started
