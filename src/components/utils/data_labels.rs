enum TimeScale {
    Relative,
    Absolute,
}

pub struct DataLabels {
    time_scale: TimeScale,
    time_last_refresh: u64,
}

impl DataLabels {
    pub fn new() -> Self {
        Self {
            time_scale: TimeScale::Relative,
            time_last_refresh: 0,
        }
    }

    pub fn labels(&self,
        current_time: u64,
        window_offset: usize,
        window_len: usize,
        refresh_rate: u64)
    {
        
    }
}