#[derive(Clone, Default, Debug)]
pub struct MemoryItem {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}

impl MemoryItem {
    pub fn new(total_memory: u64, used_memory: u64, total_swap: u64, used_swap: u64) -> Self {
        Self {
            total_memory,
            used_memory,
            total_swap,
            used_swap,
        }
    }

    pub fn update(&mut self, total_memory: u64, used_memory: u64, total_swap: u64, used_swap: u64) {
        self.total_memory = total_memory;
        self.used_memory = used_memory;
        self.total_swap = total_swap;
        self.used_swap = used_swap;
    }

    pub fn total_memory(&self) -> u64 {
        self.total_memory
    }

    pub fn used_memory(&self) -> u64 {
        self.used_memory
    }
    
    pub fn total_memory_gb(&self) -> f64 {
        self.total_memory as f64 / 1000000000_f64 
    }

    pub fn used_memory_gb(&self) -> f64 {
        self.used_memory as f64 / 1000000000_f64 
    }

    pub fn total_swap(&self) -> u64 {
        self.total_swap
    }

    pub fn used_swap(&self) -> u64 {
        self.used_swap
    }

    pub fn total_swap_gb(&self) -> f64 {
        self.total_swap as f64 / 1000000000_f64 
    }

    pub fn used_swap_gb(&self) -> f64 {
        self.used_swap as f64 / 1000000000_f64 
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
        assert_eq!(instance.total_memory_gb(), 0.0);
        assert_eq!(instance.used_memory_gb(), 0.0);
    }

    #[test]
    fn test_new() {
        let instance = MemoryItem::new(1, 2, 0, 0);
        assert_eq!(instance.total_memory(), 1);
        assert_eq!(instance.used_memory(), 2);
        assert_eq!(instance.total_memory_gb(), 0.000000001);
        assert_eq!(instance.used_memory_gb(), 0.000000002);
    }
}