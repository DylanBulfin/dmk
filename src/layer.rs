//! A layer is a mapping over all keys in a physical layout

use core::char::MAX;

use crate::{
    behavior::DefaultBehavior,
    physical_layout::{self, MAX_KEYS, PhysicalLayout},
};

/// Maximum layers that can be active in the stack at once
pub const MAX_LAYERS: usize = 10;

#[derive(Debug, Clone)]
pub struct Layer<'b, P>
where
    P: PhysicalLayout,
{
    layout: P,
    behaviors: [DefaultBehavior<'b>; physical_layout::MAX_KEYS],
}

impl<'b, P> Layer<'b, P>
where
    P: PhysicalLayout,
{
    pub fn get_behavior(&self, key: usize) -> DefaultBehavior<'b> {
        if key >= self.layout.keys() {
            panic!("Attempt to access nonexistent key")
        }

        self.behaviors[key].clone()
    }
}

pub struct LayerStack<'b, P>
where
    P: PhysicalLayout,
{
    layers: [Option<Layer<'b, P>>; MAX_LAYERS],
    len: usize,
}

impl<'b, P> LayerStack<'b, P>
where
    P: PhysicalLayout,
{
    pub fn iter(&self) -> LayerStackIter<'_, 'b, P> {
        LayerStackIter {
            stack: &self.layers,
            len: self.len,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, layer: Layer<'b, P>) {
        if self.len >= MAX_LAYERS {
            panic!("Tried to add to a full layer stack");
        }

        self.layers[self.len] = Some(layer);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<Layer<'b, P>> {
        if self.len == 0 {
            panic!("Layer stack should never be completeyl empty")
        } else if self.len == 1 {
            None
        } else {
            self.len -= 1;
            self.layers[self.len].take()
        }
    }
}

pub struct LayerStackIter<'s, 'b, P>
where
    P: PhysicalLayout,
{
    stack: &'s [Option<Layer<'b, P>>; MAX_LAYERS],
    len: usize,
    index: usize,
}

impl<'s, 'b, P> Iterator for LayerStackIter<'s, 'b, P>
where
    P: PhysicalLayout,
{
    type Item = &'s Layer<'b, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            Some(
                self.stack[self.index]
                    .as_ref()
                    .expect("Unexpectedly encountered None in layers where Some was expected"),
            )
        }
    }
}
