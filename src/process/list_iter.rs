use super::list_items_iter::ListItemsIterator;
use super::process_list_items::ProcessListItem;

pub struct ListIterator<'a> {
    item_iter: ListItemsIterator<'a>,
    selection: Option<usize>,
}

impl<'a> ListIterator<'a> {
    pub const fn new(item_iter: ListItemsIterator<'a>, selection: Option<usize>) -> Self {
        Self {
            item_iter,
            selection,
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = (&'a ProcessListItem, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.item_iter
            .next()
            .map(|(index, item)| (item, self.selection.map(|i| i == index).unwrap_or_default()))
    }
}