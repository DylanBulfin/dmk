use core::mem;

use crate::{behavior::DefaultBehavior, timer::Timer};

pub const MAX_KEYS: usize = 110; // An upper bound for the number of keys in a layout. 

pub trait PhysicalLayout {
    fn keys(&self) -> usize;
    fn get_keys(&mut self, timer: &impl Timer) -> [bool; MAX_KEYS];
}

// TODO this is effectively the same thing as the EVec definition, I should eventually genericize
// all these collections and maybe move to a new crate for reusability
#[derive(Debug)]
pub struct HeldKeyCollection {
    arr: [Option<(usize, DefaultBehavior)>; MAX_KEYS],
    len: usize,
}

#[derive(Debug)]
pub struct HeldKeyIter<'a> {
    collection: &'a HeldKeyCollection,
    index: usize,
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

    pub fn replace(&mut self, key: usize, behavior: DefaultBehavior) {
        let mut spot = None;
        for (i, (k, _)) in self.iter().enumerate() {
            if k == &key {
                spot = Some(i);
            }
        }
        if let Some(spot) = spot {
            self.arr[spot] = Some((key, behavior));
        } else {
            panic!(
                "Failed to replace HeldKey entry, did not find it in, {:?} {:?}",
                spot, behavior
            );
        }
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

    pub fn iter(&self) -> HeldKeyIter<'_> {
        HeldKeyIter {
            collection: &self,
            index: 0,
        }
    }
}

impl<'a> Iterator for HeldKeyIter<'a> {
    type Item = &'a (usize, DefaultBehavior);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.collection.len {
            None
        } else {
            let res = Some(
                self.collection.arr[self.index]
                    .as_ref()
                    .expect("Unexpected None in HeldKeyCollection"),
            );
            self.index += 1;
            res
        }
    }
}
