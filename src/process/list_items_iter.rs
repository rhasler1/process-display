use super::process_list_item::ProcessListItem;
use super::process_list_items::ProcessListItems;

pub struct ListItemsIterator<'a> {
    list: &'a ProcessListItems,
    index: usize,
    increments: Option<usize>,
    max_amount: usize,
}

impl <'a> ListItemsIterator<'a> {
    pub const fn new(list: &'a ProcessListItems, start: usize, max_amount: usize) -> Self {
        Self {
            list,
            index: start,
            increments: None,
            max_amount,
        }
    }
}

impl<'a> Iterator for ListItemsIterator<'a> {
    type Item = (usize, &'a ProcessListItem);

    // required function for Iterator
    fn next(&mut self) -> Option<Self::Item> {
        if self.increments.unwrap_or_default() < self.max_amount {

            let items = &self.list.list_items;

            let mut init = self.increments.is_none();
    
            if let Some(i) = self.increments.as_mut() {
                *i += 1;
            }
            else {
                self.increments = Some(0);
            };
    
            // can remove loop, loop is here in case we wanted to skip `invisibile values`
            // currently not implementing `visibility` for entries.
            loop {
                if !init {
                    self.index += 1;
                }
                init = false;
    
                if self.index >= self.list.list_items.len() {
                    break
                }
                // ie:
                // let elem = &items[self.index]
                // if elem.info().is_visibile() { return Some((self.index, &items[self.index])); }
                return Some((self.index, &items[self.index]));
            }
        }
        None
    }
}