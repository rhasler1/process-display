use std::io::{self};
use std::error::Error;
use config::Config;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    event::{self, EnableMouseCapture, Event, DisableMouseCapture, KeyCode}
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

pub mod app;
pub mod config;
pub mod components;
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // set up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // set up app
    let mut app = App::new();
    app.reset();
    terminal.clear()?;

    loop {
        // draw to terminal
        terminal.draw(|f| {
            match app.draw(f) {
                Ok(_state) => {} //TODO
                Err(_err) => {} //TODO (exit program)
            }
        })?;

        // process next event
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match app.event(key) {
                    Ok(state) => {
                        if !state.is_consumed() && key.code == app.config.key_config.quit {
                            break;
                        }
                    }
                    Err(_err) => {
                        app.reset();
                    }
                }
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
    return Ok(());
}