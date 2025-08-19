//! Contains an interface to interact with the microcontroller of this keyboard. I am rusty with
//! hardware programming so this side will be just a skeleton until I actually get to implementing
//! the controller and keyboard crates.
//!
//! I think the easiest way to model this is by a base set of operations on controller Pins, and
//! provide a selection of algorithms for interpreting that as physical board (the usize index that
//! is generally used to refer to keys differs by algorithm)

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
    /// Make pin with index `pin` active-high, e.g. treating high voltages as true logically
    fn set_pin_active_high(&mut self, pin: usize);
    /// Make pin with index `pin` active-high, e.g. treating high voltages as false logically
    fn set_pin_active_low(&mut self, pin: usize);
    /// Get logical state of input pin, depending on configuration (e.g. via `set_pin_active_low`,
    /// which would make logical state the opposite of physical state)
    fn get_pin_state(&self, pin: usize) -> bool;
    /// Get logical state of input pin, depending on configuration (e.g. via `set_pin_active_low`),
    /// which would make logical state the opposite of physical state)
    fn set_pin_state(&mut self, pin: usize, state: bool);

    fn set_pin_active_bool(&mut self, pin: usize, active_val: bool) {
        if active_val {
            self.set_pin_active_high(pin);
        } else {
            self.set_pin_active_low(pin);
        }
    }
}
