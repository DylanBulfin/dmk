//! Contains an interface to interact with the microcontroller of this keyboard. I am rusty with
//! hardware programming so this side will be just a skeleton until I actually get to implementing
//! the controller and keyboard crates.
//!
//! I think the easiest way to model this is by a base set of operations on controller Pins, and
//! provide a selection of algorithms for interpreting that as physical board (the usize index that
//! is generally used to refer to keys differs by algorithm)

use crate::{physical_layout::PhysicalLayout, scanning::ScanAlgorithm, timer::Timer};

pub enum PinType {
    Input,
    Output,
}

// TODO make feature-controlled or something
pub const MAX_PINS: usize = 30;

pub trait PinSet {
    /// The number of pins in this collection
    fn len(&self) -> usize;
    /// Make pin with index `pin` an input
    fn set_pin_input(&mut self, pin: usize);
    /// Make pin with index `pin` an output
    fn set_pin_output(&mut self, pin: usize);
    /// Get logical state of input pin, //depending on is_active_high
    fn get_pin_state(&self, pin: usize) -> bool;
    /// Get logical state of input pin, //depending on is_active_high
    fn set_pin_state(&mut self, pin: usize, state: bool);
}

pub struct ControllerLayout<P, A>
where
    P: PinSet,
    A: ScanAlgorithm,
{
    keys: usize,
    pins: P,
    algorithm: A,
}

impl<P, A> PhysicalLayout for ControllerLayout<P, A>
where
    P: PinSet,
    A: ScanAlgorithm,
{
    fn keys(&self) -> usize {
        self.keys
    }

    fn get_keys(&mut self, timer: &impl Timer) -> [bool; crate::physical_layout::MAX_KEYS] {
        self.algorithm.scan_pins(&mut self.pins, timer)
    }
}
