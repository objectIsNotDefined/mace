use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame,
};

pub fn render(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let title = Paragraph::new("🔨 MACE — Setup Wizard                              Step 2 / 3")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double));
    f.render_widget(title, chunks[0]);

    let progress = Paragraph::new("━━━━━━━━━━━━━━━━━━━━━━━━○─────────────  Step A  Step B  Step C")
        .alignment(Alignment::Center);
    f.render_widget(progress, chunks[1]);

    let content = Paragraph::new(
        "Scanning your environment for AI CLI tools...\n\n\
        ✅  claude        (found at /usr/local/bin/claude)\n\
        ✅  gemini        (found at /usr/local/bin/gemini)\n\
        ✅  codex         (found at /usr/local/bin/codex)\n\
        ⬜  aider         (not found)\n\n\
        Found 3 tools. Press [Enter] to continue to Command Center →"
    )
    .block(Block::default().borders(Borders::ALL).title("Step A - Auto Detection"));
    f.render_widget(content, chunks[2]);

    let footer = Paragraph::new("[↑↓] Move  [Enter] Confirm  [Esc] Prev Step  [q] Quit")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}
