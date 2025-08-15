//! Hold-tap, a fundamental behavior in any keyboard firmware

use crate::{
    behavior::Behavior,
    evec,
    event::{Event, Key, KeyEvent},
    timer::Duration,
};

pub struct HoldTap {
    decided_hold: bool,
    decided_tap: bool,
    hold: Key,
    tap: Key,
    hold_while_undecided: bool,
    duration: Duration,
    retro_tap: bool,
}

impl Behavior for HoldTap {
    fn on_press(&mut self, _ks: &super::KeyState) -> crate::event::EVec {
        if self.hold_while_undecided {
            evec![Event::key_down(self.hold)]
        } else {
            evec![Event::None]
        }
    }

    fn on_release(&mut self, ks: &super::KeyState) -> crate::event::EVec {
        // Behavior key is released, so we want to "unpress" whatever key has been sent
        if self.decided_tap {
            evec![Event::key_up(self.tap)]
        } else if self.decided_hold || self.hold_while_undecided {
            evec![Event::key_up(self.hold)]
        } else {
            evec![]
        }
    }

    fn try_get_delay(&self) -> Option<crate::timer::Duration> {
        todo!()
    }

    fn after_delay(&mut self, ks: &super::KeyState) -> crate::event::EVec {
        todo!()
    }
}
