pub mod sysinfo_service;

pub trait ListProvider<T> {
    fn fetch_items(&self) -> Vec<T>;
}