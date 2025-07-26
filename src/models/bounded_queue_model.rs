use std::collections::VecDeque;

#[derive(Default)]
pub struct BoundedQueueModel<T> {
    items: VecDeque<T>,
    capacity: usize,
}

impl<T> BoundedQueueModel<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    
    // MUTATORS
    pub fn add_item(&mut self, item: T) {
        let len = self.items.len();

        if len < self.capacity {
            self.items.push_back(item);
        }
        else if len == self.capacity {
            self.items.pop_front();
            self.items.push_back(item);
        }
        else { // should never occur 
            while len >= self.capacity {
                self.items.pop_front();
            }
            self.items.push_back(item);
        }
    }

    // GETTERS
    pub fn front(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.items.back()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn items(&self) -> &VecDeque<T> {
        &self.items
    }

    // ITERATORS
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.items.iter()
    }
}

/*
#[cfg(test)]
mod test {
    use super::BoundedQueue;
    use crate::models::items::cpu_item::CpuItem;

    /*#[test]
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
    }*/
}*/