// need two points to draw a line
pub const MIN_WINDOW_LENGTH: usize = 2;

pub struct DataWindow {
    window_offset:      usize,
    window_length:      usize,
    buffer_capacity:    usize,
}

impl DataWindow {
    pub fn new(
        window_offset: usize,
        window_length: usize,
        buffer_capacity: usize
    ) -> Option<Self>
    {
        if window_length < MIN_WINDOW_LENGTH {
            return None
        }

        if window_offset.saturating_add(window_length) > buffer_capacity {
            return None
        }

        Some(Self {
            window_offset,
            window_length,
            buffer_capacity,         
        })
    }

    pub fn window_offset(&self) -> usize {
        self.window_offset
    }

    pub fn window_length(&self) -> usize {
        self.window_length
    }

    // clamp to max offset
    pub fn pan_positive(&mut self, amount: usize) {
        let max_offset = self.buffer_capacity.saturating_sub(self.window_length);
        self.window_offset = self.window_offset
            .saturating_add(amount)
            .min(max_offset);
    }

    // clamp to 0
    pub fn pan_negative(&mut self, amount: usize) { 
        self.window_offset = self.window_offset.saturating_sub(amount);
    }

    // zoom out, adjust offset if necessary
    pub fn zoom_out(&mut self, amount: usize) {
        let max_window_length = self.buffer_capacity.saturating_sub(self.window_offset);
        if self.window_length.saturating_add(amount) > max_window_length {
            // try to pan negative
            self.pan_negative(amount);
        }
        // recalculate max_window_length
        let max_window_length = self.buffer_capacity.saturating_sub(self.window_offset);
        self.window_length = self.window_length
            .saturating_add(amount)
            .min(max_window_length);
    }

    // window length must be non-0: clamp to 2
    pub fn zoom_in(&mut self, amount: usize) {
        let min_window_length = MIN_WINDOW_LENGTH;
        self.window_length = self.window_length
            .saturating_sub(amount)
            .max(min_window_length);
    }
}