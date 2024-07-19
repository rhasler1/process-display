use std::{collections::VecDeque, io};
use super::perf_item::CpuItem;

#[derive(Default, Clone, Debug)]
pub struct CpuItems {
    pub cpu_items: VecDeque<CpuItem>,
}

impl CpuItems {
    pub fn new(list: &VecDeque<CpuItem>, max_size: usize) -> Self{
        Self {
            cpu_items: Self::create_items(list, max_size),
        }
    }

    fn create_items(list: &VecDeque<CpuItem>, max_size: usize) -> VecDeque<CpuItem> {
        let mut items = VecDeque::with_capacity(max_size);
        let mut count = 0;
        for e in list {
            if count == max_size {
                break;
            }
            let item = e.clone();
            items.push_back(item);
            count += 1;
        }
        return items;
    }

    pub fn add_item(&mut self, item: &CpuItem, max_size: usize) -> io::Result<()> {
        if self.cpu_items.len() < max_size {
            let item = item.clone();
            self.cpu_items.push_back(item);
        }
        else if self.cpu_items.len() == max_size {
            self.cpu_items.pop_front();
            let item = item.clone();
            self.cpu_items.push_back(item);
        }
        else {
            while self.cpu_items.len() >= max_size {
                self.cpu_items.pop_front();
            }
            let item = item.clone();
            self.cpu_items.push_back(item);
        }
        Ok(())
    }
}