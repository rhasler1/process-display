#[derive(PartialEq, Clone, Copy)]
pub enum ColorWheel {
    Red,
    Blue,
    Cyan,
    Green,
    LightGreen,
    Magenta,
}

impl Default for ColorWheel {
    fn default() -> Self {
        ColorWheel::Red
    }
}

impl ColorWheel {
    const ALL: [ColorWheel; 6] = [
        ColorWheel::Red,
        ColorWheel::Blue,
        ColorWheel::Cyan,
        ColorWheel::Green,
        ColorWheel::LightGreen,
        ColorWheel::Magenta,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ColorWheel::Red => "red",
            ColorWheel::Blue => "blue",
            ColorWheel::Cyan => "cyan",
            ColorWheel::Green => "green",
            ColorWheel::LightGreen => "lightgreen",
            ColorWheel::Magenta => "magenta",
        }
    }

    pub fn rotate(&mut self) {
        if let Some(idx) = Self::ALL.iter().position(|c| c == self) {
            let next_idx = (idx + 1) % Self::ALL.len();
            *self = Self::ALL[next_idx];
        }
    }

    pub fn from_index(index: usize) -> Self {
        Self::ALL[index % Self::ALL.len()]
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
