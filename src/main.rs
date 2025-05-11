mod ui;
mod repo;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture, MouseEventKind},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub struct App {
    pub commits: Vec<(String, String, Vec<String>)>,
    pub selected: usize,
    pub scroll_offset: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut commits = repo::get_commits()?;

    // Append dummy commits for testing pagination
    for i in 1..=20 {
        commits.push((
            format!("dummyhash{i:02}"),
            format!("Dummy commit message #{i}"),
            if i % 5 == 0 {
                vec![format!("feature/dummy-{i}")]
            } else {
                vec![]
            },
        ));
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        commits,
        selected: 0,
        scroll_offset: 0,
    };

    let height = terminal.size()?.height as usize - 4; // 2 margin + 2 border


    loop {
        terminal.draw(|f| {
            ui::draw_ui::<CrosstermBackend<io::Stdout>>(
                f,
                &app.commits,
                app.selected,
                app.scroll_offset,
            )
        })?;

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            },
            Event::Mouse(me) => match me.kind {
                MouseEventKind::ScrollDown => {
                    if app.selected < app.commits.len().saturating_sub(1) {
                        app.selected += 1;
        
                        if app.selected >= app.scroll_offset + height {
                            app.scroll_offset = app.selected + 1 - height;
                        }
                    }
                }
                MouseEventKind::ScrollUp => {
                    if app.selected > 0 {
                        app.selected -= 1;
        
                        if app.selected < app.scroll_offset {
                            app.scroll_offset = app.selected;
                        }
                    }
                }
                MouseEventKind::Down(_) => {
                    let y = me.row as usize;
                    let list_start = 2 + 1; // 2 margin + 1 border
                    let relative_index = y.saturating_sub(list_start);
                    let clicked_index = app.scroll_offset + relative_index;
        
                    if clicked_index < app.commits.len() {
                        app.selected = clicked_index;
        
                        // Scroll to keep it visible
                        if app.selected < app.scroll_offset {
                            app.scroll_offset = app.selected;
                        } else if app.selected >= app.scroll_offset + height {
                            app.scroll_offset = app.selected + 1 - height;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
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