use crossterm::event::{self, KeyEvent, KeyCode, Event as CEvent, KeyEventKind};
use std::sync::mpsc;
use std::{thread, time::Duration};

#[derive(Clone, Copy)]
pub struct EventConfig {
    pub exit_key: KeyCode,
    pub tick_rate: Duration,
    pub refresh_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            // not sure if needed?
            exit_key: KeyCode::Char('q'),
            tick_rate: Duration::from_millis(250),
            // note: Minimum cpu refresh time is 200 ms-- this is the lower bound of refresh_rate
            refresh_rate: Duration::from_millis(10000),
        }
    }
}

// Three possible Event type variants:
// 1. Input(K) where K is a KeyEvent -> "process this `K` Keyevent"
// 2. Tick -> "stop waiting for KeyEvent input and move on to next instruction in main"
// 3. Refresh -> "refresh system"
//
#[derive(Clone, Copy)]
pub enum Event<K> {
    Input(K),
    Tick,
    Refresh,
}

// Struct Events includes an async sender/receiver
//
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    _tx: mpsc::Sender<Event<KeyEvent>>,
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
        // link sender and receiver on mpsc channel
        let (tx, rx) = mpsc::channel();

        // thread for sending key events and tick events
        let input_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(config.tick_rate).unwrap() {
                if let Ok(event) = event::read() {
                    if let CEvent::Key(key) = event {
                        if key.kind == KeyEventKind::Press {
                            input_tx.send(Event::Input(key)).unwrap();
                        }
                    }
                }
            }
            input_tx.send(Event::Tick).unwrap();
        });

        // thread for sending refresh events -> refresh system 
        let refresh_tx = tx.clone();
        thread::spawn(move || loop {
            thread::sleep(config.refresh_rate);
            refresh_tx.send(Event::Refresh).unwrap();
        });

        // return receiver/sender pair
        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}