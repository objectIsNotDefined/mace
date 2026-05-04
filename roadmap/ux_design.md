# 🔨 MACE — Immersive Console Workstation UX Design Document

> **Document Positioning**: This document describes the complete interaction flow, interface layout, and state machine design of MACE as an "immersive TUI console workstation (similar to lazygit / k9s / btop styles)" for developer reference.

---

## Table of Contents

1. [Design Philosophy](#1-design-philosophy)
2. [Startup Entry](#2-startup-entry)
3. [Phase 1: Onboarding Wizard](#3-phase-1-onboarding-wizard)
4. [Phase 2: Command Center](#4-phase-2-command-center)
5. [Global State Machine (FSM)](#5-global-state-machine-fsm)
6. [Keyboard Navigation Specs](#6-keyboard-navigation-specs)
7. [Layout and Visual Specs](#7-layout-and-visual-specs)
8. [Implementation Mapping](#8-implementation-mapping)

---

## 1. Design Philosophy

| Principle | Description |
|-----------|-------------|
| **Zero Friction Startup** | Users only need the `mace` command, no need to memorize subcommands, the tool automatically decides which phase to enter. |
| **Context Aware** | Automatically detects if the current directory is a Git repository, auto-sniffs installed AI CLI tools. |
| **Immersive Uninterrupted** | All configurations, errors, and confirmations are completed within the TUI. It won't exit to Shell output and require re-entry. |
| **Progressive Disclosure** | The Wizard only asks the most necessary questions; advanced options (model params, consensus strategies, etc.) are left in the Command Center's config panel. |
| **Escape Hatch** | Pressing `q` / `Ctrl+C` at any stage safely exits, state is persisted to `~/.config/mace/`. |

---

## 2. Startup Entry

### 2.1 Command Signature

```bash
mace [path]
```

- **No parameter**: Uses the current working directory (`$PWD`) as the workspace.
- **With parameter**: Uses `path` as the workspace (relative or absolute).

> **Remove Subcommand Enforcement**: Original `mace run / mace init / mace audit` subcommands are retained as **advanced direct shortcuts** (CLI mode), but the bare `mace` command directly enters the TUI without requiring `init` first.

### 2.2 Startup Decision Tree

```text
mace [path]
    │
    ├─ ~/.config/mace/config.toml exists?
    │       │
    │      YES ──────────────────────────────► Directly enter [Command Center]
    │       │
    │      NO
    │       │
    │       └─► Enter [Onboarding Wizard]
    │
    └─ path is not a valid directory? ──► Show error in TUI, prompt re-entry
```

---

## 3. Phase 1: Onboarding Wizard

### 3.1 Entry Condition

Automatically triggered when `~/.config/mace/config.toml` **does not exist**.

---

### 3.2 Wizard Overall Layout

```text
╔══════════════════════════════════════════════════════════════════╗
║  🔨 MACE — Setup Wizard                              Step 2 / 3  ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                  ║
║   Step Progress Bar (ASCII progress bar)                         ║
║   ━━━━━━━━━━━━━━━━━━━━━━━━○─────────────  Step A  Step B  Step C ║
║                                                                  ║
║   ┌────────────────────────────────────────────────────────┐     ║
║   │  Main content area for current step                    │     ║
║   │  (List selection / Text input / Confirmation summary)  │     ║
║   └────────────────────────────────────────────────────────┘     ║
║                                                                  ║
╠══════════════════════════════════════════════════════════════════╣
║  [↑↓] Move  [Enter] Confirm  [Esc] Prev Step  [q] Quit           ║
╚══════════════════════════════════════════════════════════════════╝
```

---

### 3.3 Step A — Auto-Detection

**Trigger Timing**: Auto-executes immediately upon entering the Wizard, no user interaction required.

**Behavior**:
1. Concurrently run `which <tool>` in the background to detect the following candidate tools:
   `claude`, `gemini`, `aider`, `codex`, `copilot`, `gh`, `gpt`
2. TUI displays a real-time scanning animation (spinner + tool list lighting up line by line).

**UI Mockup**:
```text
  Scanning your environment for AI CLI tools...

  ✅  claude        (found at /usr/local/bin/claude)
  ✅  gemini        (found at /usr/local/bin/gemini)
  ✅  codex         (found at /usr/local/bin/codex)
  ✅  copilot       (found at /usr/local/bin/copilot)
  ⬜  aider         (not found)
  ⬜  gpt           (not found)

  Found 4 tools. Press [Enter] to continue →
```

- If **no tools are found**: Show a warning panel, prompt to install at least one Agent CLI, provide official links, and allow the user to select "Skip, configure manually" to continue.
- After detecting tools: Highlight them and auto-recommend role mapping (see Step B).

---

### 3.4 Step B — Interactive Role Setup

**Goal**: The user assigns one Agent CLI to each of the 4 core roles (`architect`, `coder`, `reviewer`, `researcher`).

**Pre-fill Rules** (based on Step A sniffing results):

| Role | Recommended Agent | Reason |
|------|-------------------|--------|
| `architect` | `claude` | Strong reasoning, good at architecture decisions |
| `coder` | `aider` / `codex` | Focus on code generation |
| `reviewer` | `gemini` | Huge context window, suitable for code review |
| `researcher` | `codex` / `gemini` | Document understanding and retrieval |

> If a recommended Agent is not installed, auto-fallback to the first installed tool.

**UI Mockup**:
```text
  Step 2/3 — Assign Roles

  Use ↑↓ to navigate, Enter to confirm each role.

  ┌─────────────────────────────────────────────────────────┐
  │  Role: architect      ★ Recommended: claude             │
  │  > [ claude        ]  ← Currently Selected              │
  │    [ gemini        ]                                    │
  │    [ codex         ]                                    │
  │    [ copilot       ]                                    │
  │    [ -- skip --    ]                                    │
  └─────────────────────────────────────────────────────────┘

  ┌─────────────────────────────────────────────────────────┐
  │  Role: coder          ★ Recommended: codex              │
  │    [ claude        ]                                    │
  │  > [ codex         ]  ← Currently Selected              │
  │    [ copilot       ]                                    │
  └─────────────────────────────────────────────────────────┘

  (2 / 4 roles configured)
```

- Users can switch focus between roles using `Tab` / `Shift+Tab`.
- The dropdown list for each role only shows tools **detected** in Step A.
- Roles can be set to `-- skip --` (unassigned for now), Wizard does not force all to be filled.

---

### 3.5 Step C — Configuration Summary & Save

**UI Mockup**:
```text
  Step 3/3 — Confirm & Save

  The following configuration will be saved to:
  ~/.config/mace/config.toml

  ┌──────────────────────────────────────────┐
  │  Agents Detected:                        │
  │    claude, gemini, codex, copilot        │
  │                                          │
  │  Role Assignments:                       │
  │    architect  →  claude                  │
  │    coder      →  codex                   │
  │    reviewer   →  gemini                  │
  │    researcher →  copilot                 │
  │                                          │
  │  Consensus Strategy:  majority-vote      │
  │  Max Rounds:          3                  │
  └──────────────────────────────────────────┘

  [Enter] Save & Launch Command Center   [e] Edit   [q] Quit
```

- Press `Enter`: Writes to `~/.config/mace/config.toml`, **seamless transition** to the Command Center (no flickering/restarts).
- Press `e`: Jump back to Step B to continue editing.
- Press `q`: Quit (config **not** saved, Wizard will still trigger next startup).

---

## 4. Phase 2: Command Center

### 4.1 Overall Layout

The Command Center is MACE's **resident main interface**, using a fixed three-area layout:

```text
┌─────────────────────────────────────────────────────────────────┐
│  HEADER BAR                                                     │
│  🔨 MACE  │  workspace: ~/code/my-project  │  4 agents ready    │
│  architect:claude  coder:codex  reviewer:gemini  researcher:... │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  WORKSPACE AREA (Dynamic Partition, see 4.2)                    │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  PROMPT BAR                                                     │
│  ▶  Refactor the Auth module to use stateless JWTs...           │
│     [Enter] Send  [Tab] Role  [Ctrl+R] History  [?] Help        │
└─────────────────────────────────────────────────────────────────┘
```

**Three Areas Details**:

| Area | Height Ratio | Responsibility |
|------|--------------|----------------|
| **Header Bar** | ~3 lines | Global state: workspace path, ready Agents count, role mapping overview |
| **Workspace Area** | Dynamic (remaining) | Task execution panels, log streams, status dashboard (see 4.2) |
| **Prompt Bar** | ~3 lines | Resident input box, similar to Cursor/ChatGPT dialog box |

---

### 4.2 Workspace Area

The workspace dynamically switches content based on the **current state**:

#### State A: Idle

When no tasks are running, displays the **Dashboard**:

```text
  ┌─────────────────────────┐  ┌─────────────────────────┐
  │  📦 Workspace           │  │  🤖 Agent Status        │
  │  Path: ~/code/proj      │  │  claude   ● ready       │
  │  Branch: main           │  │  gemini   ● ready       │
  │  Last Task: 2h ago      │  │  codex    ○ idle        │
  │  .mace.lock: ✅ clean   │  │  copilot  ○ idle        │
  └─────────────────────────┘  └─────────────────────────┘

  ┌─────────────────────────────────────────────────────┐
  │  📋 Recent Tasks                                    │
  │  #003  Refactor Auth module          ✅ done  2h ago│
  │  #002  Add JWT validation tests      ✅ done  5h ago│
  │  #001  Init project scaffold         ✅ done 1d ago │
  └─────────────────────────────────────────────────────┘
```

#### State B: Task Running (Running)

The workspace switches to a **Multi-Agent Concurrent Log Panel**, dynamically columned by active roles count:

```text
  ┌────────────── architect (claude) ──────────────┬────────────── reviewer (gemini) ───────────────┐
  │  [FSM: PLANNING]                               │  [FSM: WAITING]                                │
  │  > Analyzing Auth module structure...          │  Waiting for architect output...               │
  │  > Identified 3 JWT touchpoints                │                                                │
  │  > Generating implementation plan...           │                                                │
  │                                                │                                                │
  ├────────────── coder (codex) ───────────────────┴────────────── researcher (copilot) ────────────┤
  │  [FSM: EXECUTING]                              │  [FSM: IDLE]                                   │
  │  > Scaffolding new JWT middleware...           │  No task assigned.                             │
  │  > Writing src/auth/jwt.rs                     │                                                │
  │  > Tests: 3/7 passing                          │                                                │
  └────────────────────────────────────────────────┴───────────────────────────────────────────────┘
```

- **Column Rules**:
  - 1 active role → Single column fullscreen
  - 2 active roles → Left/Right split
  - 3-4 active roles → 2x2 grid
  - 5+ active roles → Max 2 columns, extra Agents queue in tabs (`[` / `]`)

- Each panel implements **virtual scrolling**, can be independently focused and scrolled (`Tab` switches focus, `↑↓` scrolls).

#### State C: Consensus Conflict (Consensus Gate)

When multiple Agents have semantic conflicts, workspace switches to **Conflict Arbitration Panel**:

```text
  ╔══════════ ⚡ CONSENSUS CONFLICT DETECTED ══════════╗
  ║                                                    ║
  ║  architect (claude) proposed:                      ║
  ║    fn verify_jwt(token: &str) -> Result<Claims>    ║
  ║                                                    ║
  ║  coder (codex) implemented:                        ║
  ║    fn validate_token(t: String) -> Option<Claims>  ║
  ║                                                    ║
  ║  Semantic diff: Signature mismatch (AST level)     ║
  ║  Round 2 / 3 max                                   ║
  ║                                                    ║
  ║  [a] Accept architect  [c] Accept coder            ║
  ║  [d] Show full diff    [h] Human-in-the-Loop       ║
  ╚════════════════════════════════════════════════════╝
```

#### State D: Human-in-the-Loop (HITL)

If consensus exceeds `max_rounds`, workspace shows HITL UI, freezing all automation:

```text
  ⚠️  HUMAN-IN-THE-LOOP REQUIRED

  Task #004 has reached consensus deadlock after 3 rounds.
  All sandboxes (git worktrees) are preserved.

  Options:
  > [1] Review diff in detail
    [2] Accept architect's version
    [3] Accept coder's version  
    [4] Write custom resolution
    [5] Abort task (cleanup worktrees)
```

---

### 4.3 Prompt Bar (Resident Input)

**Interactions**:

| Action | Function |
|--------|----------|
| Direct typing | Freeform task description (sent to current active roles combination) |
| `@architect` | Prefix command, individually invoke a role |
| `Tab` | Pop up role selector (select roles involved in this task) |
| `Ctrl+R` | Open history tasks list (fuzzy search) |
| `Ctrl+K` | Open Command Palette (like VS Code `Ctrl+Shift+P`) |
| `Enter` | Dispatch task, switch workspace to "Running" state |
| `Esc` | Cancel input / Close auxiliary panels |

**Role Selector (Tab Popup)**:
```text
  Select roles for this task:
  [x] architect (claude)
  [x] coder     (codex)
  [ ] reviewer  (gemini)   ← Press Space to toggle
  [ ] researcher(copilot)

  [Enter] Confirm  [Esc] Cancel
```

---

### 4.4 Command Palette (Ctrl+K)

```text
  ┌───────────────────────────────────────────────────────┐
  │  > _                                                  │
  ├───────────────────────────────────────────────────────┤
  │  ▶  Open Config Editor        Ctrl+,                  │
  │     Switch Workspace          Ctrl+W                  │
  │     View Audit Log (.mace.lock)                       │
  │     Resume Last Task                                  │
  │     Run mace doctor                                   │
  │     Toggle Theme (dark/light)                         │
  │     Help & Shortcuts          ?                       │
  └───────────────────────────────────────────────────────┘
```

---

## 5. Global State Machine (FSM)

```text
              ┌──────────────────────┐
   mace       │                      │
  ─────────►  │    BOOT / CHECK      │
              │  (config exists?)    │
              └──────────┬───────────┘
                         │
              ┌──────────▼───────────┐         ┌─────────────────┐
              │  WIZARD              │ ──────► │  IDLE           │
              │  Step A/B/C          │  save   │ (Command Center)│
              └──────────────────────┘         └────────┬────────┘
                                                        │ Enter (prompt)
                                               ┌────────▼────────┐
                                               │  DISPATCHING    │
                                               │  (DAG route,    │
                                               │   worktrees)    │
                                               └────────┬────────┘
                                                        │
                                               ┌────────▼────────┐
                                               │  RUNNING        │◄──┐
                                               │  (Concurrent    │   │  retry
                                               │   Agent Exec)   │   │
                                               └────────┬────────┘   │
                                                        │            │
                                            ┌───────────▼───┐        │
                                            │ CONSENSUS GATE│        │
                                            │ (AST conflict?)        │
                                            └──┬────────┬───┘        │
                                            PASS     CONFLICT        │
                                               │        │            │
                                      ┌────────▼┐  ┌────▼─────────┐  │
                                      │  MERGING│  │  DEBATE      │──┘
                                      │  & DONE │  │ (max rounds?)│
                                      └─────────┘  └──────┬───────┘
                                                    EXCEED│
                                                   ┌──────▼──────────┐
                                                   │  HITL           │
                                                   │ (Human in Loop) │
                                                   └─────────────────┘
```

---

## 6. Keyboard Navigation Specs

### Global Shortcuts (Active in all states)

| Shortcut | Function |
|----------|----------|
| `q` / `Ctrl+C` | Quit MACE (can choose to keep/cancel running tasks) |
| `?` | Show help panel |
| `Ctrl+K` | Open command palette |
| `Ctrl+,` | Open config editor |
| `Ctrl+Z` | Pause current task (keep worktrees) |

### Workspace Navigation

| Shortcut | Function |
|----------|----------|
| `Tab` | Switch focus between Agent panels |
| `↑↓` | Scroll within focused panel |
| `[` / `]` | Switch Agent tabs (if 5+ Agents) |
| `f` | Fullscreen current focused Agent panel |

### Prompt Bar

| Shortcut | Function |
|----------|----------|
| `/` | Quick focus Prompt Bar from anywhere |
| `Ctrl+R` | Search task history |
| `Tab` | Role selector |
| `Enter` | Send task |

---

## 7. Layout and Visual Specs

### 7.1 Color Semantics

| Color | Semantics | Usage |
|-------|-----------|-------|
| Cyan | Primary | Header, border highlight, Prompt Bar focus |
| Green | Success | Agent ready, task done, ✅ |
| Yellow | Warning / Progress | Consensus debate, spinner |
| Red | Error / Conflict | CONFLICT, HITL, error messages |
| DarkGray | Unfocused | Inactive Agent panel borders |

### 7.2 Character Set Specs

Use pure ASCII + Box-drawing characters, ensure cross-platform terminal compatibility:
- Borders: `─` `│` `┌` `┐` `└` `┘` `├` `┤` `╔` `╗` `╚` `╝` `║`
- State indicators: `●` (active) `○` (idle) `⚡` (conflict) `⚠️` (warning)
- Progress: `━` `─` `○` (step progress bar)

### 7.3 Responsive Layout

MACE detects actual terminal dimensions before rendering:

| Terminal Width | Layout Adjustment |
|----------------|-------------------|
| `< 80` cols | Single column mode, all panels stacked vertically |
| `80–120` cols | Standard 2-column grid |
| `> 120` cols | Widescreen mode, Header displays more metadata |

---

## 8. Implementation Mapping

### Relationship with Existing Code

| Module | Existing Implementation | Required Additions/Modifications |
|--------|-------------------------|----------------------------------|
| Config Loading | `src/config.rs` (Done) | No change needed |
| Tool Sniffing | `config::detect_tools()` (Done) | Port to Wizard Step A |
| CLI Entry | `src/cli.rs` (Subcommand mode) | Add zero-param TUI entry (`mace [path]`) |
| TUI Framework | Not implemented | New `src/tui/` module |
| Wizard Flow | Not implemented | New `src/tui/wizard.rs` |
| Command Center | Not implemented | New `src/tui/command_center.rs` |
| Agent Panels | Not implemented | New `src/tui/panels/agent_panel.rs` |
| Prompt Bar | Not implemented | New `src/tui/components/prompt_bar.rs` |

### Recommended Module Structure

```text
src/
├── main.rs                    # Entry: Parse [path], route to Wizard / CC
├── cli.rs                     # Keep advanced subcommands (run/audit/resume)
├── config.rs                  # Config load & sniffing (Complete)
└── tui/
    ├── mod.rs                 # TUI main loop, event routing
    ├── wizard/
    │   ├── mod.rs             # Wizard FSM
    │   ├── step_a_detect.rs   # Env sniffer page
    │   ├── step_b_roles.rs    # Role assignment page
    │   └── step_c_confirm.rs  # Confirmation page
    ├── command_center/
    │   ├── mod.rs             # Command Center layout
    │   ├── header.rs          # Header Bar
    │   ├── workspace.rs       # Workspace state routing
    │   ├── prompt_bar.rs      # Prompt input box
    │   └── panels/
    │       ├── dashboard.rs   # Idle dashboard
    │       ├── agent_panel.rs # Agent log panel
    │       ├── conflict.rs    # Consensus conflict panel
    │       └── hitl.rs        # HITL panel
    └── components/
        ├── spinner.rs         # Loading animation
        ├── list_select.rs     # Arrow key list select
        └── command_palette.rs # Ctrl+K command palette
```

### Corresponding Roadmap Phase

| Document Module | Roadmap Phase |
|-----------------|---------------|
| Startup + Wizard | Phase 1 (Observer) — Tool registry, TUI setup |
| CC Idle Panel | Phase 1 (Observer) — TUI Dashboard |
| Agent Log Panel | Phase 1 (Observer) — Multi-pane monitoring |
| Conflict Panel | Phase 3 (Consensus) — MCDD engine |
| HITL Panel | Phase 3 (Consensus) — Human intervention UI |
| DAG Dispatch | Phase 4 (Governance) — DAG routing |

---

*Document Version v0.1 — 2026-05-05*  
*Author: Janber Zhang*
