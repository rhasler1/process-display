pub mod items;
pub mod bounded_queue_model;
pub mod vec_model;

pub trait Filterable {
    fn matches_filter(&self, filter: &str) -> bool;
}

pub trait Sortable<S> {
    fn cmp_with(&self, other: &Self, sort: &S) -> std::cmp::Ordering;
}