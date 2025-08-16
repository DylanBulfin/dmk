use core::ops::Index;

use crate::{
    event::queue::EventQueue,
    layer::{Layer, LayerStack},
    physical_layout::PhysicalLayout,
    virtual_board::VirtualKeyboard,
};

pub struct State<'b, P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<'b, P>>,
{
    layers: C,
    layout: P,
    virtual_board: VirtualKeyboard,
    layer_stack: LayerStack<'b, P>,
    event_queue: EventQueue<'b>,
}
