use std::collections::VecDeque;
use std::io;

use crate::performance::cpu_perf_info::CpuInfo;
use crate::performance::perf_item::CpuItem;

use super::DrawableComponent;

#[derive(Default, Clone, Debug)]
pub struct PerformanceComponent {
    cpu_info: CpuInfo,
}

impl PerformanceComponent {
    pub fn new(list: &VecDeque<CpuItem>, max_size: usize) -> Self {
        Self {
            cpu_info: CpuInfo::new(list, max_size)
        }
    }

    pub fn refresh(&mut self, item: &CpuItem) -> io::Result<()> {
        self.cpu_info.add_item(item)?;
        Ok(())
    }
}

impl DrawableComponent for PerformanceComponent {
    fn draw(&mut self, _f: &mut ratatui::Frame, _area: ratatui::prelude::Rect, _focused: bool) -> io::Result<()> {
        Ok(())
    }
}