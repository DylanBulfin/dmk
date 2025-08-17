pub mod hold_tap;
pub mod key_press;

use crate::{
    behavior::{hold_tap::HoldTap, key_press::KeyPress},
    evec,
    event::EVec,
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
    NoArg(NoArgBehavior),
    Arg(ArgBehavior),
}

/// A behavior that captures other behaviors. These behaviors cannot be captured themselves. E.g.
/// HoldTap behavior can capture other behaviors for its hold and tap functionality, but cannot be
/// captured itself. It can only capture no-arg behaviors like KeyPress
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgBehavior {
    HoldTap(HoldTap),
}

impl From<HoldTap> for ArgBehavior {
    fn from(value: HoldTap) -> Self {
        Self::HoldTap(value)
    }
}

/// A behavior that doesn't capture any other behaviors. These are the only behaviors valid to be
/// captured. So, the HoldTap behavior can capture one behavior for the hold function and one for
/// the tap function. But you cannot have a nested HoldTap function for instance. This is because
/// only No-arg behaviors can be captured as arguments in this way
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoArgBehavior {
    KeyPress(KeyPress),
    None,
    Transparent,
}

impl From<KeyPress> for NoArgBehavior {
    fn from(value: KeyPress) -> Self {
        Self::KeyPress(value)
    }
}

impl From<ArgBehavior> for DefaultBehavior {
    fn from(value: ArgBehavior) -> Self {
        Self::Arg(value)
    }
}

impl From<NoArgBehavior> for DefaultBehavior {
    fn from(value: NoArgBehavior) -> Self {
        Self::NoArg(value)
    }
}

impl Behavior for ArgBehavior {
    fn on_press(&mut self, ks: &KeyState) -> EVec {
        match self {
            ArgBehavior::HoldTap(hold_tap) => hold_tap.on_press(ks),
        }
    }

    fn on_release(&mut self, ks: &KeyState) -> EVec {
        match self {
            ArgBehavior::HoldTap(hold_tap) => hold_tap.on_release(ks),
        }
    }

    fn try_get_delay(&self) -> Option<Duration> {
        match self {
            ArgBehavior::HoldTap(hold_tap) => hold_tap.try_get_delay(),
        }
    }

    fn after_delay(&mut self, ks: &KeyState) -> EVec {
        match self {
            ArgBehavior::HoldTap(hold_tap) => hold_tap.after_delay(ks),
        }
    }
}

impl Behavior for NoArgBehavior {
    fn on_press(&mut self, ks: &KeyState) -> EVec {
        match self {
            NoArgBehavior::KeyPress(key_press) => key_press.on_press(ks),
            _ => evec![],
        }
    }

    fn on_release(&mut self, ks: &KeyState) -> EVec {
        match self {
            NoArgBehavior::KeyPress(key_press) => key_press.on_release(ks),
            _ => evec![],
        }
    }

    fn try_get_delay(&self) -> Option<Duration> {
        match self {
            NoArgBehavior::KeyPress(key_press) => key_press.try_get_delay(),
            _ => None,
        }
    }

    fn after_delay(&mut self, ks: &KeyState) -> EVec {
        match self {
            NoArgBehavior::KeyPress(key_press) => key_press.after_delay(ks),
            _ => evec![],
        }
    }
}

impl Behavior for DefaultBehavior {
    fn on_press(&mut self, ks: &KeyState) -> EVec {
        match self {
            DefaultBehavior::NoArg(nab) => nab.on_press(ks),
            DefaultBehavior::Arg(ab) => ab.on_press(ks),
        }
    }

    fn on_release(&mut self, ks: &KeyState) -> EVec {
        match self {
            DefaultBehavior::NoArg(nab) => nab.on_release(ks),
            DefaultBehavior::Arg(ab) => ab.on_release(ks),
        }
    }

    fn try_get_delay(&self) -> Option<Duration> {
        match self {
            DefaultBehavior::NoArg(nab) => nab.try_get_delay(),
            DefaultBehavior::Arg(ab) => ab.try_get_delay(),
        }
    }

    fn after_delay(&mut self, ks: &KeyState) -> EVec {
        match self {
            DefaultBehavior::NoArg(nab) => nab.after_delay(ks),
            DefaultBehavior::Arg(ab) => ab.after_delay(ks),
        }
    }
}

impl From<HoldTap> for DefaultBehavior {
    fn from(value: HoldTap) -> Self {
        Self::Arg(ArgBehavior::HoldTap(value.clone()))
    }
}

impl From<KeyPress> for DefaultBehavior {
    fn from(value: KeyPress) -> Self {
        Self::NoArg(NoArgBehavior::KeyPress(value))
    }
}
