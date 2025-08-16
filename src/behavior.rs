pub mod key_press;
pub mod mod_tap;

use crate::{
    behavior::{key_press::KeyPress, mod_tap::HoldTap},
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
pub enum DefaultBehavior {
    HoldTap(HoldTap),
    KeyPress(KeyPress),
}
