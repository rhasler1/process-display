use std::io;
use super::{perf_item::CpuItem, perf_items::CpuItems};

#[derive(Default, Clone, Debug)]
pub struct CpuInfo {
    pub cpu_items: CpuItems,
    max_size: usize,
}

impl CpuInfo {
    pub fn new(max_size: usize) -> Self {
        Self {
            cpu_items: CpuItems::default(),
            max_size,
        }
    }

    pub fn add_item(&mut self, item: &CpuItem) -> io::Result<()> {
        self.cpu_items.add_item(item, self.max_size)?;
        Ok(())
    }

    pub fn max_size(&self) -> usize {
        self.max_size.clone()
    }

    pub fn back(&self) -> Option<&CpuItem> {
        return self.cpu_items.cpu_items.back()
    }
}
