use std::io::{self};
use std::error::Error;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    event::{self, EnableMouseCapture, Event, KeyEvent, DisableMouseCapture, KeyCode}
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    prelude::*
};
pub mod app;
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
                Ok(_state) => {}
                Err(_err) => {}
            }
        })?;

        // process next event
        if let Event::Key(key) = event::read()? {
            // app.event(key) should return Ok(true) if keyevent has been consumed and
            // Ok(false) if keyevent has not been consumed
            if key.kind == event::KeyEventKind::Press {
                match app.event(key) {
                    Ok(state) => {
                        // if keyevent has not been consumed and keycode is 'q' break loop and exit app
                        if state && key.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                    // if error occurred when procesing app, then restart app
                    Err(_err) => {
                        app.reset()?;
                    }
                }
            }
        }
    }
    // exit app gracefully
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    return Ok(());
}
