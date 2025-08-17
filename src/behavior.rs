pub mod hold_tap;
pub mod key_press;
pub mod momentary_layer;

use crate::{
    behavior::{hold_tap::HoldTap, key_press::KeyPress, momentary_layer::MomentaryLayer},
    evec,
    event::EVec,
    timer::Duration,
};

pub struct KeyState {}

pub trait Behavior {
    // Unique identifier for each behavior, allows easy detemination of which behaviors are held
    // fn id(&self) -> u64;

    fn on_press(&mut self, ks: &KeyState) -> EVec;

    fn on_release(&mut self, ks: &KeyState) -> EVec;
    // Add call_time arg which allows Behavior to statelessly determine whether the delay function
    // has timed out, as long sa it takes the Instant of creation in its constructor
    // fn on_release(&mut self, call_time: Instant, ks: &KeyState) -> EVec;

    fn try_get_delay(&self) -> Option<Duration>;

    fn after_delay(&mut self, ks: &KeyState) -> EVec;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefaultBehavior {
    NoArg(NoArgBehavior),
    Arg(ArgBehavior),
}

macro_rules! make_arg_behaviors {
    ($(($varid:ident, $vartype:ty)),+) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        /// A behavior that captures other behaviors. These behaviors cannot be captured themselves. E.g.
        /// HoldTap behavior can capture other behaviors for its hold and tap functionality, but cannot be
        /// captured itself. It can only capture no-arg behaviors like KeyPress
        pub enum ArgBehavior {
            $($varid($vartype)),+,
        }

        $(
        impl From<$vartype> for ArgBehavior {
            fn from(value: $vartype) -> Self {
                Self::$varid(value)
            }
        }

        impl From<$vartype> for DefaultBehavior {
            fn from(value: $vartype) -> Self {
                Self::Arg(ArgBehavior::$varid(value.into()))
            }
        }
        )+

        impl Behavior for ArgBehavior {
            fn on_press(&mut self, ks: &KeyState) -> EVec {
                match self {
                    $(ArgBehavior::$varid(val) => val.on_press(ks)),+
                }
            }

            fn on_release(&mut self, ks: &KeyState) -> EVec {
                match self {
                    $(ArgBehavior::$varid(val) => val.on_release(ks)),+
                }
            }

            fn try_get_delay(&self) -> Option<Duration> {
                match self {
                    $(ArgBehavior::$varid(val) => val.try_get_delay()),+
                }
            }

            fn after_delay(&mut self, ks: &KeyState) -> EVec {
                match self {
                    $(ArgBehavior::$varid(val) => val.after_delay(ks)),+
                }
            }
        }
    };
}

macro_rules! make_noarg_behaviors {
    ($(($varid:ident, $vartype:ty)),+) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        /// A behavior that captures no other behaviors (such as KeyPress or MomentaryLayer.)
        pub enum NoArgBehavior {
            $($varid($vartype)),+,
            None,
            Transparent,
        }

        $(
        impl From<$vartype> for NoArgBehavior {
            fn from(value: $vartype) -> Self {
                Self::$varid(value)
            }
        }

        impl From<$vartype> for DefaultBehavior {
            fn from(value: $vartype) -> Self {
                Self::NoArg(NoArgBehavior::$varid(value.into()))
            }
        }
        )+

        impl Behavior for NoArgBehavior {
            fn on_press(&mut self, ks: &KeyState) -> EVec {
                match self {
                    $(NoArgBehavior::$varid(val) => val.on_press(ks)),+,
                    NoArgBehavior::None => evec![],
                    NoArgBehavior::Transparent => evec![],
                }
            }

            fn on_release(&mut self, ks: &KeyState) -> EVec {
                match self {
                    $(NoArgBehavior::$varid(val) => val.on_release(ks)),+,
                    NoArgBehavior::None => evec![],
                    NoArgBehavior::Transparent => evec![],
                }
            }

            fn try_get_delay(&self) -> Option<Duration> {
                match self {
                    $(NoArgBehavior::$varid(val) => val.try_get_delay()),+,
                    NoArgBehavior::None => None,
                    NoArgBehavior::Transparent => None,
                }
            }

            fn after_delay(&mut self, ks: &KeyState) -> EVec {
                match self {
                    $(NoArgBehavior::$varid(val) => val.after_delay(ks)),+,
                    NoArgBehavior::None => evec![],
                    NoArgBehavior::Transparent => evec![],
                }
            }
        }
    };
}

make_noarg_behaviors!((KeyPress, KeyPress), (MomentaryLayer, MomentaryLayer));
make_arg_behaviors!((HoldTap, HoldTap));

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
