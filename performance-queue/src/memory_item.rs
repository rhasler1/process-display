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
    }

    #[test]
    fn test_new() {
        let instance = MemoryItem::new(1, 2, 3, 4);
        assert_eq!(instance.total_memory(), 1);
        assert_eq!(instance.used_memory(), 2);
        assert_eq!(instance.free_memory(), 3);
        assert_eq!(instance.available_memory(), 4);
    }
}