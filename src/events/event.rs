use crossterm::event::{self, KeyEventKind, Event as CEvent};
use std::{thread, time::Duration, sync::mpsc};
use crate::input::{Key, Mouse};

#[derive(Clone, Copy)]
pub struct EventConfig {
    pub tick_rate: Duration,
    pub refresh_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
            refresh_rate: Duration::from_millis(10000),
        }
    }
}

#[derive(Clone)]
pub enum Event {
    KeyInput(Key),
    MouseInput(Mouse),
    Tick,
    Refresh,
}

pub struct Events {
    rx: mpsc::Receiver<Event>,
    _tx: mpsc::Sender<Event>,
}

impl Events {
    pub fn new(tick_rate: u64, refresh_rate: u64) -> Events {
        Events::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
            refresh_rate: Duration::from_millis(refresh_rate),
            ..Default::default()
        })
    }

    pub fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();
        let tick_tx = tx.clone();
        let refresh_tx = tx.clone();

        thread::spawn(move || loop {
            if event::poll(config.tick_rate).unwrap() {
                if let Ok(event) = event::read() {
                    if let CEvent::Key(key) = event {
                        // guard needed for Windows
                        if key.kind == KeyEventKind::Press {
                            input_tx.send(Event::KeyInput(Key::from(key))).unwrap();
                        }
                    }
                    if let CEvent::Mouse(mouse) = event {
                        input_tx.send(Event::MouseInput(Mouse::from(mouse))).unwrap();
                    }
                }
            }
        });

        thread::spawn(move || loop {
            thread::sleep(config.tick_rate);
            tick_tx.send(Event::Tick).unwrap();
        });

        thread::spawn(move || loop {
            thread::sleep(config.refresh_rate);
            refresh_tx.send(Event::Refresh).unwrap();
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}