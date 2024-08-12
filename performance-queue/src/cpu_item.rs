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
    use super::CpuItem;

    #[test]
    fn test_default() {
        let instance = CpuItem::default();
        assert_eq!(instance.global_usage(), 0.0);
        assert_eq!(instance.num_cores(), None);
        assert_eq!(instance.frequency(), 0);
        assert!(instance.brand().is_empty());
    }

    #[test]
    fn test_new() {
        let instance = CpuItem::new(1.0, Some(11), 4056, String::from("Apple"));
        assert_eq!(instance.global_usage(), 1.0);
        assert_eq!(instance.num_cores(), Some(11));
        assert_eq!(instance.frequency(), 4056);
        assert_eq!(instance.brand(), String::from("Apple"));
    }
}