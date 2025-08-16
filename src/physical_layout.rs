use core::cell::OnceCell;

pub const MAX_KEYS: usize = 110; // An upper bound for the number of keys in a layout. 

pub trait PhysicalLayout {
    fn keys(&self) -> usize;
}
