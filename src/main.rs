use anyhow::Result;
use std::io;
use crossterm::ExecutableCommand;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    event::DisableMouseCapture
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crate::events::event::{Event, Events};
use crate::app::App;

pub mod app;
pub mod config;
pub mod components;
pub mod events;
pub mod models;

// If the program's view of the World is incorrect, crash the program, don't hide false beliefs.
fn main() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let config = config::Config::default();
    let tick_rate = config.tick_rate();
    let refresh_rate = config.refresh_rate();
    
    let events = Events::new(tick_rate, refresh_rate);

    let mut app = App::new(config);

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            match app.draw(f) {
                Ok(_state) => {}
                Err(err) => {
                    println!("error: {}", err.to_string());
                    std::process::exit(1);
                }
            }
        })?;

        match events.next()? {
            Event::Input(key) => match app.key_event(key) {
                Ok(state) => {
                    if !state.is_consumed() && key.code == app.config.key_config.exit {
                        break;
                    }
                }
                Err(err) => {
                    app.error.set(err.to_string())?;
                }
            }
            Event::Refresh => match app.refresh_event() {
                Ok(_state) => {}
                Err(err) => {
                    app.error.set(err.to_string())?;
                } 
            }
            Event::Tick => {
                continue
            }
        }
    }

    // tear down terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}
