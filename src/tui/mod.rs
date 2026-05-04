pub mod wizard;
pub mod command_center;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

pub enum AppState {
    Wizard,
    CommandCenter,
}

pub async fn run_tui() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState::Wizard;

    // run app
    let res = run_app(&mut terminal, &mut state);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
) -> Result<(), Box<dyn Error>> where <B as ratatui::backend::Backend>::Error: 'static {
    loop {
        terminal.draw(|f| {
            match state {
                AppState::Wizard => wizard::render(f),
                AppState::CommandCenter => command_center::render(f),
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Enter => {
                    match state {
                        AppState::Wizard => *state = AppState::CommandCenter,
                        AppState::CommandCenter => {}
                    }
                }
                _ => {}
            }
        }
    }
}
