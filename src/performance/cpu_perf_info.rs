use std::{collections::VecDeque, io};

use super::{perf_item::CpuItem, perf_items::CpuItems};

#[derive(Default, Clone, Debug)]
pub struct CpuInfo {
    cpu_items: CpuItems,
    max_size: usize,
}

impl CpuInfo {
    pub fn new(list: &VecDeque<CpuItem>, max_size: usize) -> Self {
        Self {
            cpu_items: CpuItems::new(list, max_size),
            max_size,
        }
    }

    pub fn add_item(&mut self, item: &CpuItem) -> io::Result<()> {
        self.cpu_items.add_item(item, self.max_size)?;
        Ok(())
    }
}

