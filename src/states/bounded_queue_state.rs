use crate::models::bounded_queue_model::BoundedQueueModel;
use std::collections::VecDeque;

pub struct BoundedQueueState<T> {
    model: BoundedQueueModel<T>,
    selection: Option<usize>,
}

impl <T> BoundedQueueState<T> {
    pub fn new(
        capacity: usize,
        selection: Option<usize>,
    ) -> Self
    {
        let model = BoundedQueueModel::new(capacity);

        Self {
            model,
            selection,
        }
    }

    // MUTATORS
    pub fn set_selection(&mut self, selection: Option<usize>) {
        self.selection = selection;
    }

    pub fn add_item(&mut self, item: T) {
        self.model.add_item(item);
    }

    // GETTERS
    pub fn front(&self) -> Option<&T> {
        self.model.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.model.back()
    }

    pub fn capacity(&self) -> usize {
        self.model.capacity()
    }

    pub fn model_items(&self) -> &VecDeque<T> {
        self.model.items()
    }

    // ITERS
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.model.iter()
    }
}