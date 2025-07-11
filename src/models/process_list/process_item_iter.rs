use super::process_item::ProcessItem;

pub struct ProcessItemIterator<'a> {
    list: &'a Vec<ProcessItem>,
    selection: Option<usize>,
    start_idx: usize,
    end_idx: usize,
}

impl<'a> ProcessItemIterator<'a> {
    pub fn new(list: &'a Vec<ProcessItem>, selection: Option<usize>, start_idx: usize, max_iter: usize) -> Self {
        let end_idx = usize::min(start_idx + max_iter, list.len());
        Self {
            list,
            selection,
            start_idx,
            end_idx,
        }
    }
}

impl<'a> Iterator for ProcessItemIterator<'a> {
    type Item = (&'a ProcessItem, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_idx >= self.end_idx {
            return None;
        }

        let item = &self.list[self.start_idx];
        let is_selected = Some(self.start_idx) == self.selection;

        self.start_idx += 1;

        Some((item, is_selected))
    }
}