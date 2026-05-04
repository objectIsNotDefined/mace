use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame,
};

pub fn render(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let header = Paragraph::new("🔨 MACE  │  workspace: ~/code/my-project  │  4 agents ready\narchitect:claude  coder:codex  reviewer:gemini  researcher:copilot")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("HEADER BAR").border_type(BorderType::Rounded));
    f.render_widget(header, chunks[0]);

    let workspace_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let dashboard_left = Paragraph::new("📦 Workspace\nPath: ~/code/proj\nBranch: main\nLast Task: 2h ago\n.mace.lock: ✅ clean")
        .block(Block::default().borders(Borders::ALL).title("Workspace"));
    f.render_widget(dashboard_left, workspace_chunks[0]);

    let dashboard_right = Paragraph::new("🤖 Agent Status\nclaude   ● ready\ngemini   ● ready\ncodex    ○ idle\ncopilot  ○ idle")
        .block(Block::default().borders(Borders::ALL).title("Agent Status"));
    f.render_widget(dashboard_right, workspace_chunks[1]);

    let prompt_bar = Paragraph::new("▶  Refactor the Auth module to use stateless JWTs...\n[Enter] Send  [Tab] Role  [Ctrl+R] History  [?] Help")
        .block(Block::default().borders(Borders::ALL).title("PROMPT BAR"));
    f.render_widget(prompt_bar, chunks[2]);
}
