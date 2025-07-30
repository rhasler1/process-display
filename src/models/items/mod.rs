use std::ops::Div;

pub mod cpu_item;
pub mod memory_item;
pub mod temp_item;
pub mod process_item;
pub mod network_item;

pub fn byte_to_kb(data: u64) -> u64 {
    data.div(1024)
}

pub trait ByteToMB {
    fn byte_to_mb(self) -> Self;
}

macro_rules! impl_byte_to_mb {
    ($($t:ty),*) => {
        $(
            impl ByteToMB for $t {
                #[inline]
                fn byte_to_mb(self) -> Self {
                    self / 1048576
                }
            }
        )*
    };
}
impl_byte_to_mb!(u32, u64, usize);
