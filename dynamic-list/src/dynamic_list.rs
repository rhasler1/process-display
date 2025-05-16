pub struct DynamicList<T> {
    items: Vec<T>,
}

impl<T> DynamicList<T> {
    pub fn new() -> Self {
        DynamicList {
            items: Vec::new()
        }
    }

    pub fn filter<F>(&self, predicate: F) -> DynamicList<T> where
        F: Fn(&T) -> bool,
        T: Clone,
    {
        let filtered_items = self.items.iter().filter(|i| predicate(i)).cloned().collect();
        DynamicList {
            items: filtered_items
        }
    }

    pub fn replace_all(&mut self, vec: Vec<T>) {
        self.items = vec
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        DynamicList {
            items: vec
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn sort_by<F>(&mut self, compare: F) where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.items.sort_by(compare)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}
