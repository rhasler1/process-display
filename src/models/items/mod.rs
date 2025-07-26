use std::ops::Div;

pub mod cpu_item;
pub mod memory_item;
pub mod temp_item;
pub mod process_item;
pub mod network_item;

pub fn byte_to_kb(data: u64) -> u64 {
    data.div(1024)
}

pub fn byte_to_mb(data: u64) -> u64 {
    data.div(1048576)
}