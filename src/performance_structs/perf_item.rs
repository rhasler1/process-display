#[derive(Clone, Default, Debug)]
pub struct CpuItem {
    total_usage: f32,
    num_cores: Option<usize>,
    frequency: u64,
    brand: String,
}

impl CpuItem {
    pub fn new(total_usage: f32, num_cores: Option<usize>, frequency: u64, brand: String) -> Self {
        Self {
            total_usage,
            num_cores,
            frequency,
            brand,
        }
    }
    
    pub fn total_usage(&self) -> f32 {
        self.total_usage.clone()
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