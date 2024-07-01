use crossterm::event::{self, KeyEvent, KeyCode};
use std::sync::mpsc;
use std::{thread, time::Duration};

// largely inspired from: https://github.com/TaKO8Ki/gobang/blob/7b1b5f7eba3ea98a5d254a12ea31f383ee7737d1/src/event/events.rs

#[derive(Clone, Copy)]
pub struct EventConfig {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            // not sure if needed?
            exit_key: KeyCode::Char('q'),
            // note: Minimum cpu refresh time is 200 ms-- this is the lower bound of tick_rate
            tick_rate: Duration::from_millis(2000),
        }
    }
}

// Two possible Event type variants:
// 1. Input(I) where I is a KeyEvent
// 2. Tick
//
#[derive(Clone, Copy)]
pub enum Event<I> {
    Input(I),
    Tick,
}

// Struct Events includes an async sender/receiver
//
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    _tx: mpsc::Sender<Event<KeyEvent>>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Events {
        Events::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    pub fn with_config(config: EventConfig) -> Events {
        // link sender and receiver on mpsc channel
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || loop {
            // if there is an event available and it is a KeyEvent send event over channel
            if event::poll(config.tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    event_tx.send(Event::Input(key)).unwrap();
                }
            }
            // send a tick event 
            event_tx.send(Event::Tick).unwrap();
        });
        // return receiver/sender pair
        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}