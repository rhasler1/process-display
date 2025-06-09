#[derive(Clone, Default, Debug)]
pub struct TempItem {
    //TODO
    temp: f32,
    max_temp: f32,
    critical_temp: f32,
    label: String,
}

impl TempItem {
    pub fn new(temp: f32, max_temp: f32, critical_temp: f32, label: String) -> Self {
        Self {
            temp,
            max_temp,
            critical_temp,
            label
        }
    }

    pub fn temp(&self) -> f32 {
        self.temp
    }

    pub fn max_temp(&self) -> f32 {
        self.max_temp
    }

    pub fn critical_temp(&self) -> f32 {
        self.critical_temp
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}