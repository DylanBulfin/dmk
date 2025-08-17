//! A layer is a mapping over all keys in a physical layout

use core::{char::MAX, ops::Index};

use crate::{
    behavior::{DefaultBehavior, NoArgBehavior},
    physical_layout::{self, MAX_KEYS, PhysicalLayout},
};

/// Maximum layers that can be active in the stack at once
pub const MAX_LAYERS: usize = 10;

#[derive(Debug, Clone)]
pub struct Layer<P>
where
    P: PhysicalLayout,
{
    layout: P,
    behaviors: [DefaultBehavior; physical_layout::MAX_KEYS],
}

impl<P> Layer<P>
where
    P: PhysicalLayout,
{
    pub fn get_behavior(&self, key: usize) -> DefaultBehavior {
        if key >= self.layout.keys() {
            panic!("Attempt to access nonexistent key")
        }

        self.behaviors[key].clone()
    }
}

#[derive(Debug)]
pub struct LayerStack<P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    all_layers: C,
    base_layer: usize,
    layers: [Option<usize>; MAX_LAYERS],
    len: usize,
}

impl<P, C> LayerStack<P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    pub fn iter(&self) -> LayerStackIter<'_, P, C> {
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
            if layer.get_behavior(key) != NoArgBehavior::Transparent.into() {
                return layer.get_behavior(key);
            }
        }

        NoArgBehavior::None.into()
    }
}

impl<P, C> Index<usize> for LayerStack<P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    type Output = Layer<P>;

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

pub struct LayerStackIter<'s, P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    stack: &'s LayerStack<P, C>,
    len: usize,
    index: usize,
}

impl<'s, P, C> Iterator for LayerStackIter<'s, P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    type Item = &'s Layer<P>;

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
        behavior::{Behavior, NoArgBehavior, key_press::KeyPress},
        key::Key,
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

        let mut bb = [NoArgBehavior::None.into(); MAX_KEYS];
        bb[0] = NoArgBehavior::KeyPress(KeyPress::new(Key::A)).into();
        bb[1] = NoArgBehavior::KeyPress(KeyPress::new(Key::B)).into();
        bb[2] = NoArgBehavior::KeyPress(KeyPress::new(Key::C)).into();

        let mut b1 = [NoArgBehavior::None.into(); MAX_KEYS];
        b1[0] = NoArgBehavior::KeyPress(KeyPress::new(Key::D)).into();
        b1[1] = NoArgBehavior::Transparent.into();
        b1[2] = NoArgBehavior::KeyPress(KeyPress::new(Key::F)).into();

        let mut b2 = [NoArgBehavior::None.into(); MAX_KEYS];
        b2[0] = NoArgBehavior::Transparent.into();
        b2[1] = NoArgBehavior::Transparent.into();
        b2[2] = NoArgBehavior::KeyPress(KeyPress::new(Key::I)).into();

        let bl = Layer {
            layout,
            behaviors: bb,
        };
        let l1 = Layer {
            layout,
            behaviors: b1,
        };
        let l2 = Layer {
            layout,
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
