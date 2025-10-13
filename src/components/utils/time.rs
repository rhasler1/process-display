use chrono::{DateTime, Duration, Local};

pub struct TimeUtil {
    now: DateTime<Local>,
}

impl TimeUtil {
    pub fn new() -> Self {
        let now = Local::now();

        Self {
            now,
        }
    }

    pub fn refresh(&mut self) {
        self.now = Local::now();
    }

    
}