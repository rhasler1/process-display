// Simple wrapper over Vector
pub struct VecModel<T> {
    items: Vec<T>
}

impl <T> VecModel<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
        }
    }

    // MUTATORS
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn replace(&mut self, new_items: Vec<T>) {
        self.items = new_items;
    }

    // GETTERS
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }
}