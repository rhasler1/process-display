pub mod app;
pub mod config;
pub mod components;
pub mod events;
pub mod process;

use std::io;
use std::error::Error;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // terminal setup::begin
    setup_terminal()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    // terminal setup::end
    
    // event handler setup::begin
    let events = Events::new(250, 5000); // argument 1: tick_rate , argument 2: system refresh_rate
    // event handler setup::end

    // app creation and initialization::begin
    let mut app = App::new();
    app.init().await?;
    // app creation and initialization::end

    terminal.clear()?; // clear terminal

    // main event loop::begin
    loop {

        // draw to terminal::begin
        terminal.draw(|f| {
            match app.draw(f) {
                Ok(_state) => {}
                Err(_err) => {}
            }
        })?;
        // draw to terminal::end

        // process next event::begin
        match events.next()? {

            // Input Key Event
            Event::Input(key) => match app.event(key).await {
                Ok(state) => {
                    if !state.is_consumed() && key.code == app.config.key_config.quit {
                        break;
                    }
                }
                Err(_err) => {
                    //app.reset();
                }
            }

            // Refresh Event
            Event::Refresh => match app.refresh().await {
                Ok(_state) => {}
                Err(_err) => {} 
            }

            // Tick Event
            Event::Tick => {}
        }
        // process next event::end
    }
    // main event loop:: end

    // tear down terminal::begin
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // tear down terminal::end

    return Ok(())
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}