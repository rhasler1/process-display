#[derive(Clone, Default, Debug)]
pub struct MemoryItem {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
    available_memory: u64,
}

impl MemoryItem {
    pub fn new(total_memory: u64, used_memory: u64, free_memory: u64, available_memory: u64) -> Self {
        Self {
            total_memory,
            used_memory,
            free_memory,
            available_memory,
        }
    }

    pub fn total_memory(&self) -> u64 {
        return self.total_memory.clone()
    }

    pub fn used_memory(&self) -> u64 {
        return self.used_memory.clone()
    }

    pub fn free_memory(&self) -> u64 {
        return self.free_memory.clone()
    }

    pub fn available_memory(&self) -> u64 {
        return self.available_memory.clone()
    }
    
    pub fn total_memory_gb(&self) -> f64 {
        self.total_memory as f64 / 1000000000 as f64 
    }

    pub fn used_memory_gb(&self) -> f64 {
        self.used_memory as f64 / 1000000000 as f64 
    }

    pub fn free_memory_gb(&self) -> f64 {
        self.free_memory as f64 / 1000000000 as f64 
    }

    pub fn available_memory_gb(&self) -> f64 {
        self.available_memory as f64 / 1000000000 as f64 
    }
}

#[cfg(test)]
mod test {
    use super::MemoryItem;

    #[test]
    fn test_default() {
        let instance = MemoryItem::default();
        assert_eq!(instance.total_memory(), 0);
        assert_eq!(instance.used_memory(), 0);
        assert_eq!(instance.free_memory(), 0);
        assert_eq!(instance.available_memory(), 0);
        assert_eq!(instance.total_memory_gb(), 0.0);
        assert_eq!(instance.used_memory_gb(), 0.0);
        assert_eq!(instance.free_memory_gb(), 0.0);
        assert_eq!(instance.available_memory_gb(), 0.0);
    }

    #[test]
    fn test_new() {
        let instance = MemoryItem::new(1, 2, 3, 4);
        assert_eq!(instance.total_memory(), 1);
        assert_eq!(instance.used_memory(), 2);
        assert_eq!(instance.free_memory(), 3);
        assert_eq!(instance.available_memory(), 4);
        assert_eq!(instance.total_memory_gb(), 0.000000001);
        assert_eq!(instance.used_memory_gb(), 0.000000002);
        assert_eq!(instance.free_memory_gb(), 0.000000003);
        assert_eq!(instance.available_memory_gb(), 0.000000004);
    }
}