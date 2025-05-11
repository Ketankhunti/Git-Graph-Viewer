mod repo;
mod ui;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commits = repo::get_commits()?; // Get commit list

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| ui::draw_ui::<CrosstermBackend<io::Stdout>>(f, &commits))?;
        
    // Quit on `q`
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
