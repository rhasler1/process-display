use std::io;
use std::error::Error;

//use anyhow::Result;
//use anyhow::Ok;
use crossterm::ExecutableCommand;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    event::{self, EnableMouseCapture, DisableMouseCapture}
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

pub mod app;
pub mod config;
pub mod components;
pub mod events;

use crate::events::event::{Event, Events};
use crate::app::App;

// TODO:
// 1. Read more on async programming
// 2. determine which methods need to be labeled as async
// 3. draw diagram of async tasks/execution


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_terminal()?;

    let mut stderr = io::stderr();
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new(250, 5000); // tick rate-- system refreshes every `argument` ms
    let mut app = App::new();
    app.reset();
    terminal.clear()?;

    loop {
        // draw to terminal
        terminal.draw(|f| {
            match app.draw(f) {
                Ok(_state) => {}
                Err(_err) => {}
            }
        })?;
        // process next event
        match events.next()? {
            // match key event and process
            Event::Input(key) => match app.event(key).await {
                Ok(state) => {
                    if !state.is_consumed() && key.code == app.config.key_config.quit {
                        break;
                    }
                }
                Err(_err) => {
                    app.reset();
                }
            }
            // match tick event and process
            Event::Tick => {}
            Event::Refresh => match app.event_tick().await {
                Ok(_state) => {}
                Err(_err) => {} 
            }
        }
        // update structures
        app.update().await?;
    }
    // tear down terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    return Ok(())
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}