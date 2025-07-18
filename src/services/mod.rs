pub mod sysinfo_service;

// trait VecProvider<T> details:
//
// VecProvider<T> is implemented in SysInfoService for each item `T` in
// models/items/. The served vectors are used in creating & refreshing(updating)
// models (e.g., vec_model.rs). For more information on refreshing models
// see the trait Refreshable<S> in components/mod.rs.
//
pub trait VecProvider<T> {
    fn fetch_items(&self) -> Vec<T>;
}