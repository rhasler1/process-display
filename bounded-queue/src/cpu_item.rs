#[derive(Clone, Default, Debug)]
pub struct CpuItem {
    id: usize,
    usage: f32,
    frequency: u64,
    name: String,
    brand: String,
    vendor_id: String,
}

impl CpuItem {
    pub fn new(
        id: usize,
        usage: f32,
        frequency: u64,
        name: String,
        brand: String,
        vendor_id: String,
    ) -> Self {
        Self {
            id,
            usage,
            frequency,
            name,
            brand,
            vendor_id,
        }
    }

    pub fn usage(&self) -> f32 {
        self.usage
    }

    pub fn id(&self) -> usize {
        self.id
    }
    
    pub fn global_usage(&self) -> f32 {
        self.usage
    }


    pub fn frequency(&self) -> u64 {
        self.frequency
    }

    pub fn brand(&self) -> String {
        self.brand.clone()
    }
}

#[cfg(test)]
mod test {
    use super::CpuItem;
/* 
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
    */
}
