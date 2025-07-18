use crate::models::vec_model::VecModel;
use crate::models::{Filterable, Sortable};

pub struct VecState<T, S> {
    model: VecModel<T>,
    selection: Option<usize>,
    sort: Option<S>,
    filter: Option<String>,
}

impl <T, S> VecState<T, S> 
where S: Clone,
{
    pub fn new(model: Vec<T>, selection: Option<usize>, sort: Option<S>, filter: Option<String>) -> Self {
        let model = VecModel::new(model);

        Self {
            model,
            selection,
            sort,
            filter,
        }
    }

    // MUTATORS
    pub fn set_selection(&mut self, selection: Option<usize>) {
        self.selection = selection;
    }

    pub fn set_sort(&mut self, sort: Option<S>) {
        self.sort = sort;
    }

    pub fn set_filter(&mut self, filter: Option<&str>) {
        self.filter = if let Some(filter) = filter {
            Some(String::from(filter))
        }
        else {
            None
        };
    }

    // ACCESS TO MODEL MUTATORS
    pub fn push(&mut self, item: T) {
        self.model.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.model.pop()
    }

    pub fn clear(&mut self) {
        self.model.clear();
    }

    pub fn replace(&mut self, new_items: Vec<T>) {
        self.model.replace(new_items);
    }

    // GETTERS
    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn is_empty(&self) -> bool {
        self.model.is_empty()
    }

    pub fn list(&self) -> &[T] {
        &self.model.items()
    }

    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    pub fn sort(&self) -> &Option<S> {
        &self.sort
    }

    pub fn filter(&self) -> Option<&str> {
        self.filter.as_deref()
    }
}

impl <T, S> VecState<T, S>
where
    T: Filterable + Sortable<S>
{
    // Vec<usize> mapping viewable indices(e.g., rows when rendering a table) -> immutable model indices after sort/filter
    pub fn view_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.model.items().len()).collect();

        if let Some(filter) = &self.filter {
            indices.retain(|&i| self.model.items()[i].matches_filter(filter));
        }

        if let Some(sort) = &self.sort {
            indices.sort_by(|&i, &j| self.model.items()[i].cmp_with(&self.model.items()[j], sort));
        }

        indices
    }

    // Returns an iterator where Item = (usize:"mapping to immutable model index",
    // &T:"reference to current item", bool:"does the current item map to the selected index?")
    pub fn iter_with_selection(&self) -> impl Iterator<Item = (usize, &T, bool)> {
        let indices = self.view_indices();
        let selected = self.selection;

        let res = indices.into_iter().map(move |i| {
            let is_selected = Some(i) == selected;
            (i, &self.model.items()[i], is_selected)
        });
        
        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::models::items::process_item::{ProcessItem, ProcessItemSortOrder};
    use super::*;

    #[test]
    fn test() {
        let items: Vec<ProcessItem> = vec![
            ProcessItem::new(0, String::from("Discord"), 12.0, 12, 12, 12, 12, String::from("Runnable"), String::from("test/")),
            ProcessItem::new(9, String::from("Discord-Helper"), 20.0, 20, 20, 20, 20, String::from("Runnable"), String::from("test/")),
            ProcessItem::new(3, String::from("iTerm"), 9.0, 9, 9, 9, 9, String::from("Runnable"), String::from("test/")),
            ProcessItem::new(11, String::from("process-display"), 2.0, 2, 2, 2, 2, String::from("Runnable"), String::from("test/")),
            ];
        
        let sort: Option<ProcessItemSortOrder> = Some(ProcessItemSortOrder::CpuUsageDec);
        let selection: Option<usize> = Some(0);
        let filter: Option<String> = None;

        let list: VecState<ProcessItem, ProcessItemSortOrder> = VecState::new(items, selection, sort, filter);
        
        let view = list.iter_with_selection();
        view.for_each(|(idx, item, sel)| {
            
            let text = format!("idx={}, item.name={}, sel={}", idx, item.name(), sel);
            println!("{text}");
        });
    }
}