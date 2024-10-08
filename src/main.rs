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

pub mod app;
pub mod config;
pub mod components;
pub mod events;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // terminal setup
    setup_terminal()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    // event handler setup
    // argument 1: tick_rate , argument 2: system refresh_rate
    let events = Events::new(250, 5000);

    // app creation and initialization
    let config = config::Config::default();
    let mut app = App::new(config);
    app.refresh().await?;

    // clear terminal
    terminal.clear()?;

    // main event loop
    loop {
        // draw to terminal
        terminal.draw(|f| {
            match app.draw(f) {
                Ok(_state) => {}
                Err(err) => {
                    println!("error: {}", err.to_string());
                    std::process::exit(1);
                }
            }
        })?;

        // process next event
        match events.next()? {
            Event::Input(key) => match app.event(key).await {
                Ok(state) => {
                    if !state.is_consumed() && key.code == app.config.key_config.exit_popup {
                        break;
                    }
                }
                Err(err) => {
                    app.error.set(err.to_string())?;
                }
            }
            Event::Refresh => match app.refresh().await {
                Ok(_state) => {}
                Err(err) => {
                    app.error.set(err.to_string())?;
                } 
            }
            Event::Tick => {}
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

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}