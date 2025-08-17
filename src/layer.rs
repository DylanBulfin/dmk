//! A layer is a mapping over all keys in a physical layout

use core::{char::MAX, ops::Index};

use crate::{
    behavior::DefaultBehavior,
    physical_layout::{self, MAX_KEYS, PhysicalLayout},
};

/// Maximum layers that can be active in the stack at once
pub const MAX_LAYERS: usize = 10;

#[derive(Debug, Clone)]
pub struct Layer<P>
where
    P: PhysicalLayout,
{
    id: usize,
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

pub struct LayerStack<P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    all_layers: C,
    base_layer: Layer<P>,
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
        if self.index >= self.len + 1 {
            None
        } else if self.index == self.len {
            let ret = Some(&self.stack.base_layer);
            self.index += 1;
            ret
        } else {
            let ret = Some(&self.stack[self.index]);
            self.index += 1;
            ret
        }
    }
}
