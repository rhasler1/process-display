#[derive(Clone, Default, Debug)]
pub struct CpuItem {
    global_usage: f32,
    num_cores: Option<usize>,
    frequency: u64,
    brand: String,
}

impl CpuItem {
    pub fn new(global_usage: f32, num_cores: Option<usize>, frequency: u64, brand: String) -> Self {
        Self {
            global_usage,
            num_cores,
            frequency,
            brand,
        }
    }
    
    pub fn global_usage(&self) -> f32 {
        self.global_usage.clone()
    }

    pub fn num_cores(&self) -> Option<usize> {
        self.num_cores.clone()
    }

    pub fn frequency(&self) -> u64 {
        self.frequency.clone()
    }

    pub fn brand(&self) -> String {
        self.brand.clone()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cpuinfo_default() {
    }
}