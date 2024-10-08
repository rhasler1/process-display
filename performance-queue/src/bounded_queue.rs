use std::{collections::VecDeque, io};

pub struct PerformanceQueue<T> {
    pub performance_items: VecDeque<T>,
    max_size: usize,
}

// Clone trait is required for T to clone elements when adding items.
impl<T: Clone> PerformanceQueue<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            performance_items: VecDeque::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn add_item(&mut self, item: &T) -> io::Result<()> {
        if self.performance_items.len() < self.max_size {
            let item = item.clone();
            self.performance_items.push_back(item);
        }
        else if self.performance_items.len() == self.max_size {
            self.performance_items.pop_front();
            let item = item.clone();
            self.performance_items.push_back(item);
        }
        else {
            while self.performance_items.len() >= self.max_size {
                self.performance_items.pop_front();
            }
            let item = item.clone();
            self.performance_items.push_back(item);
        }
        Ok(())
    }

    pub fn front(&self) -> Option<&T> {
        return self.performance_items.front()
    }

    pub fn back(&self) -> Option<&T> {
        return self.performance_items.back()
    }

    pub fn max_size(&self) -> usize {
        self.max_size.clone()
    }
}

#[cfg(test)]
mod test {
    use super::PerformanceQueue;
    use crate::CpuItem;

    #[test]
    fn test_bounded_queue() {
        let mut instance: PerformanceQueue<CpuItem> = PerformanceQueue::new(2);
        assert_eq!(instance.max_size(), 2);
        let cpu_item_1 = CpuItem::new(2.0, Some(11), 4056, String::from("Apple"));
        let cpu_item_2 = CpuItem::new(13.2, Some(11), 4056, String::from("Apple"));
        let cpu_item_3 = CpuItem::new(15.7, Some(11), 4056, String::from("Apple"));
        let _ = instance.add_item(&cpu_item_1);
        assert_eq!(instance.back().unwrap().global_usage(), 2.0);
        assert_eq!(instance.front().unwrap().global_usage(), 2.0);
        let _ = instance.add_item(&cpu_item_2);
        assert_eq!(instance.back().unwrap().global_usage(), 13.2);
        assert_eq!(instance.front().unwrap().global_usage(), 2.0);
        let _ = instance.add_item(&cpu_item_3);
        assert_eq!(instance.back().unwrap().global_usage(), 15.7);
        assert_eq!(instance.front().unwrap().global_usage(), 13.2);
    }
}