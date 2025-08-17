pub const MAX_KEYS: usize = 110; // An upper bound for the number of keys in a layout. 

pub trait PhysicalLayout {
    fn keys(&self) -> usize;
    fn get_status(&self, key: usize) -> bool;
    fn get_arr_copy(&self) -> [bool; MAX_KEYS];
}
