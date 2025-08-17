//! A layer is a mapping over all keys in a physical layout

use core::ops::Index;

use crate::{
    behavior::{DefaultBehavior, NoArgBehavior},
    physical_layout::{self, PhysicalLayout},
};

/// Maximum layers that can be active in the stack at once
pub const MAX_LAYERS: usize = 10;

#[derive(Debug)]
pub struct Layer {
    keys: usize,
    behaviors: [Option<DefaultBehavior>; physical_layout::MAX_KEYS],
}

impl Layer {
    pub fn new(
        keys: usize,
        behaviors: [Option<DefaultBehavior>; physical_layout::MAX_KEYS],
    ) -> Self {
        Self { keys, behaviors }
    }

    pub fn get_behavior(&self, key: usize) -> Option<DefaultBehavior> {
        if key >= self.keys {
            panic!("Attempt to access nonexistent key")
        }

        self.behaviors[key].clone()
    }
}

#[derive(Debug)]
pub struct LayerStack<C>
where
    C: Index<usize, Output = Layer>,
{
    all_layers: C,
    base_layer: usize,
    layers: [Option<usize>; MAX_LAYERS],
    len: usize,
}

impl<C> LayerStack<C>
where
    C: Index<usize, Output = Layer>,
{
    pub fn new(all_layers: C) -> Self {
        Self {
            all_layers,
            base_layer: 0,
            layers: [None; MAX_LAYERS],
            len: 0,
        }
    }

    pub fn iter(&self) -> LayerStackIter<'_, C> {
        LayerStackIter {
            stack: &self,
            len: self.len,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, layer: usize) {
        if self.len >= MAX_LAYERS {
            panic!("Tried to add to a full layer stack");
        }

        self.layers[self.len] = Some(layer);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            self.layers[self.len].take()
        }
    }

    /// Pop layers off the stack until you have popped the indicated layer, and then stop.
    /// INCLUSIVE
    pub fn pop_until(&mut self, layer: usize) {
        while let Some(l) = self.pop() {
            if l == layer {
                return;
            }
        }
    }

    pub fn find_key_behavior(&self, key: usize) -> DefaultBehavior {
        for layer in self.iter() {
            if let Some(behavior) = layer.get_behavior(key)
                && behavior != NoArgBehavior::Transparent.into()
            {
                return layer
                    .get_behavior(key)
                    .expect("Attempted to get value of invalid key");
            }
        }

        NoArgBehavior::None.into()
    }
}

impl<C> Index<usize> for LayerStack<C>
where
    C: Index<usize, Output = Layer>,
{
    type Output = Layer;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Out of bounds in LayerStack, len is {} and index is {}",
                self.len, index
            );
        }
        &self.all_layers[self.layers[index].unwrap_or_else(|| {
            panic!(
                "Unexpected none at index {} in LayerStack when len is {}",
                index, self.len
            )
        })]
    }
}

pub struct LayerStackIter<'s, C>
where
    C: Index<usize, Output = Layer>,
{
    stack: &'s LayerStack<C>,
    len: usize,
    index: usize,
}

impl<'s, C> Iterator for LayerStackIter<'s, C>
where
    C: Index<usize, Output = Layer>,
{
    type Item = &'s Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.len {
            None
        } else {
            let ret = if self.index == self.len {
                // Base layer special case
                Some(&self.stack.all_layers[self.stack.base_layer])
            } else {
                Some(
                    &self.stack.all_layers[self.stack.layers[self.len - 1 - self.index]
                        .unwrap_or_else(|| {
                            panic!(
                                "Unexpected None when parsing LayerStackIter, i: {}, len: {}",
                                self.index, self.len
                            )
                        })],
                )
            };

            self.index += 1;
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        behavior::{NoArgBehavior, key_press::KeyPress},
        key::Key,
        physical_layout::MAX_KEYS,
    };

    use super::*;

    #[test]
    fn test_layer_basics() {
        #[derive(Clone, Copy, Debug)]
        struct Layout {
            arr: [bool; MAX_KEYS],
        }

        impl PhysicalLayout for Layout {
            fn keys(&self) -> usize {
                3
            }

            fn get_status(&self, key: usize) -> bool {
                self.arr[key]
            }

            fn get_arr_copy(&self) -> [bool; MAX_KEYS] {
                self.arr
            }
        }

        let layout = Layout {
            arr: [false; MAX_KEYS],
        };

        let mut bb = [None; MAX_KEYS];
        bb[0] = Some(NoArgBehavior::KeyPress(KeyPress::new(Key::A)).into());
        bb[1] = Some(NoArgBehavior::KeyPress(KeyPress::new(Key::B)).into());
        bb[2] = Some(NoArgBehavior::KeyPress(KeyPress::new(Key::C)).into());

        let mut b1 = [None; MAX_KEYS];
        b1[0] = Some(NoArgBehavior::KeyPress(KeyPress::new(Key::D)).into());
        b1[1] = Some(NoArgBehavior::Transparent.into());
        b1[2] = Some(NoArgBehavior::KeyPress(KeyPress::new(Key::F)).into());

        let mut b2 = [None; MAX_KEYS];
        b2[0] = Some(NoArgBehavior::Transparent.into());
        b2[1] = Some(NoArgBehavior::Transparent.into());
        b2[2] = Some(NoArgBehavior::KeyPress(KeyPress::new(Key::I)).into());

        let bl = Layer {
            keys: layout.keys(),
            behaviors: bb,
        };
        let l1 = Layer {
            keys: layout.keys(),
            behaviors: b1,
        };
        let l2 = Layer {
            keys: layout.keys(),
            behaviors: b2,
        };

        let mut layers = [None; MAX_LAYERS];
        layers[0] = Some(1);
        layers[1] = Some(2);

        let stack = LayerStack {
            all_layers: [bl, l1, l2],
            base_layer: 0,
            layers,
            len: 2,
        };

        assert_eq!(
            stack.find_key_behavior(0),
            NoArgBehavior::KeyPress(KeyPress::new(Key::D)).into()
        );
        assert_eq!(
            stack.find_key_behavior(1),
            NoArgBehavior::KeyPress(KeyPress::new(Key::B)).into()
        );
        assert_eq!(
            stack.find_key_behavior(2),
            NoArgBehavior::KeyPress(KeyPress::new(Key::I)).into()
        );
    }
}
