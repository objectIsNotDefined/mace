# Phase 1: Observer (Terminal & Process Monitoring Layer) Design Document

## 🎯 Core Objectives
Implement basic command-line process invocation, output hijacking, and display the parallel execution status of multiple Agents through a TUI (Ratatui). MACE does not call LLMs directly; instead, it acts as a scheduler to launch third-party AI CLIs such as `claude-code`, `aider`, etc.

## 🏗️ Architecture Design & Technology Stack

### 1. Configuration & CLI Parsing (`config` & `cli`)
*   **Responsibility**: Parse user-input commands (`run`, `audit`, `init`) and the `.mace.toml` configuration file in the project root.
*   **Dependencies (Crates)**: `clap` (CLI parsing), `serde`, `serde_derive`, `toml` (Config parsing).
*   **Core Data Structures**:
    *   `MaceConfig`: Maps to the global configuration in `.mace.toml`.
    *   `RoleDef`: Defines configuration for individual Agent roles (Agent type, model, system prompt templates, etc.).

### 2. Tool Registry & Environment Diagnostics (`registry`)
*   **Responsibility**: Scan the local environment to detect installed AI CLIs, verify their availability, and provide an interactive command to list, configure, or troubleshoot tools.
*   **Mechanism**:
    *   Provide a command like `mace tools` or `mace doctor` to display a matrix of supported vs. installed agents.
    *   Proactively execute commands (e.g., `aider --version`) or check `$PATH` to ensure binaries are executable before launching tasks.
    *   Allow users to map custom binary paths for specific agents in `.mace.toml`.

### 3. Cross-Platform Process Scheduling (`process`)
*   **Responsibility**: Provide fully asynchronous, non-blocking child process management based on `tokio::process`. Support launching multiple AI CLI processes simultaneously.
*   **Mechanism**:
    *   Allocate an independent `Command` instance for each Agent.
    *   Hijack `stdin`, `stdout`, and `stderr` using `Stdio::piped()`.
    *   Force-inject environment variables upon startup (e.g., `TERM=dumb`, `NO_COLOR=1`) to minimize interactive animation output from AI CLIs.

### 4. I/O Hijacking & Stream Cleaning (`io`)
*   **Responsibility**: Read the child process output streams in real-time, clean them, and distribute them to the TUI components.
*   **Challenge**: Certain AI tools (like Aider) heavily utilize PTY (Pseudo-Terminal) features and ANSI escape codes. Reading them directly results in garbled text on the TUI interface.
*   **Solution**:
    *   Introduce the `strip-ansi-escapes` library as a middleware for asynchronous streams to filter all control characters.
    *   Implement a simple heuristic parser to extract "valid dialogue/thinking information" from the raw stream and discard "progress bar rendering" information.

### 5. TUI Dashboard Construction (`ui`)
*   **Responsibility**: Provide a multi-pane terminal monitoring interface similar to `tmux`.
*   **Dependencies (Crates)**: `ratatui` (UI rendering framework), `crossterm` (Terminal backend abstraction).
*   **UI Layout Design**:
    *   **Header**: Display a global overview of the current task, the number of running Agents, and total elapsed time.
    *   **Split Panes**: Dynamically split the terminal window based on the number of concurrent Agents (e.g., left/right columns). Each pane scrolls the log of its respective Agent in real-time.
    *   **Footer/Status Bar**: Shortcut hints (e.g., `Ctrl+C` to interrupt, `Tab` to switch focus).

## 📝 Detailed Task Breakdown & Status Tracking

- [ ] **1. CLI Basic Framework Setup**
  - [ ] 1.1 Initialize the Rust project structure and introduce `clap`.
  - [ ] 1.2 Implement the `mace init` command to generate a default `.mace.toml` template.
  - [ ] 1.3 Implement parsing and deserialization logic for the configuration file.
- [ ] **2. Tool Registry & Environment Diagnostics Development**
  - [ ] 2.1 Develop the `mace tools` / `mace doctor` CLI command to display tool status.
  - [ ] 2.2 Implement background probing functions to check `$PATH` and binary versions (e.g., `aider`, `claude`).
  - [ ] 2.3 Throw early validation errors if a task requests an uninstalled tool.
- [ ] **3. Asynchronous Child Process Manager Development**
  - [ ] 3.1 Encapsulate the `AgentRunner` struct to support spawning arbitrary executables.
  - [ ] 3.2 Implement asynchronous `tokio` task streams for reading `stdout/stderr`.
  - [ ] 3.3 Test invoking `aider --message "test"` and successfully capture the output.
- [ ] **4. Output Cleaning Pipeline Construction**
  - [ ] 3.1 Integrate the ANSI cleaning tool.
  - [ ] 3.2 Establish an `mpsc` (Multi-Producer Single-Consumer) message channel mechanism with the TUI to ensure the logging thread does not block the UI refresh thread.
- [ ] **5. TUI Monitoring Dashboard Development**
  - [ ] 4.1 Setup the `ratatui` + `crossterm` Event Loop.
  - [ ] 4.2 Implement an adaptive grid layout (dynamic 1x2, 2x2 split-screen based on child process count).
  - [ ] 4.3 Implement automatic scrolling and coloring for each pane (coloring based on cleaned information levels).

---
**Current Phase Status**: ⏳ Not Started
