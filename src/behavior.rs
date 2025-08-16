pub mod hold_tap;
pub mod key_press;

use crate::{
    behavior::{hold_tap::HoldTap, key_press::KeyPress},
    event::{EVec, Event},
    timer::Duration,
};

pub struct KeyState {}

pub trait Behavior {
    fn on_press(&mut self, ks: &KeyState) -> EVec;

    fn on_release(&mut self, ks: &KeyState) -> EVec;

    fn try_get_delay(&self) -> Option<Duration>;

    fn after_delay(&mut self, ks: &KeyState) -> EVec;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefaultBehavior<'b> {
    HoldTap(HoldTap<'b>),
    KeyPress(KeyPress),
}

impl<'b> From<&'b HoldTap<'b>> for DefaultBehavior<'b> {
    fn from(value: &'b HoldTap) -> Self {
        Self::HoldTap(value.clone())
    }
}

impl<'b> From<KeyPress> for DefaultBehavior<'b> {
    fn from(value: KeyPress) -> Self {
        Self::KeyPress(value)
    }
}
