use crate::behavior::DefaultBehavior;

pub const MAX_KEYS: usize = 110; // An upper bound for the number of keys in a layout. 

pub trait PhysicalLayout {
    fn keys(&self) -> usize;
    fn get_status(&self, key: usize) -> bool;
    fn get_arr_copy(&self) -> [bool; MAX_KEYS];
}

// TODO this is effectively the same thing as the EVec definition, I should eventually genericize
// all these collections and maybe move to a new crate for reusability
pub struct HeldKeyCollection {
    arr: [Option<(usize, DefaultBehavior)>; MAX_KEYS],
    len: usize,
}

impl HeldKeyCollection {
    pub fn new() -> Self {
        Self {
            arr: [None; _],
            len: 0,
        }
    }

    pub fn push(&mut self, key: usize, behavior: DefaultBehavior) {
        if self.len >= MAX_KEYS {
            panic!("Attempt to add held key to full collection")
        }

        self.arr[self.len] = Some((key, behavior));
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> (usize, DefaultBehavior) {
        if index >= self.len {
            panic!();
        }

        let ret = self.arr[index];

        for i in index + 1..self.len {
            self.arr[i - 1] = self.arr[i];
        }

        self.len -= 1;

        self.arr[self.len] = None;

        ret.expect("Unexpected None when index < len")
    }

    pub fn try_remove_key(&mut self, key: usize) -> Option<DefaultBehavior> {
        // TODO rewrite this to use binary search
        for i in 0..self.len {
            if let Some((k, _)) = self.arr[i] {
                if key == k {
                    let (_, behavior) = self.remove(i);
                    return Some(behavior);
                }
            } else {
                panic!("None at index {} when len is {}", i, self.len);
            }
        }

        None
    }
}
