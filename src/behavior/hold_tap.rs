//! Hold-tap, a fundamental behavior in any keyboard firmware

use crate::{
    behavior::{Behavior, NoArgBehavior},
    evec,
    event::{EVec, Event},
    timer::Duration,
};

// May need to change this lifetime logic. I have to solidify a lifetime eventually and I think if
// I have a  generic in the overall state func that will try to solidify it and, given
// behaviors are actually spawned from random places all the time, this may cause problems. The
// alternate is to enforce that behaviors have static lifetimes. I generate all behaviors in a
// const context maybe?
//
// TODO
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoldTap {
    decided_hold: bool,
    decided_tap: bool,
    hold: NoArgBehavior,
    tap: NoArgBehavior,
    hold_while_undecided: bool,
    duration: Duration,
}

impl HoldTap {
    pub fn new(
        hold: NoArgBehavior,
        tap: NoArgBehavior,
        duration: Duration,
        hold_while_undecided: bool,
    ) -> Self {
        Self {
            decided_hold: false,
            decided_tap: false,
            hold,
            tap,
            hold_while_undecided,
            duration,
        }
    }
}

impl Behavior for HoldTap {
    fn on_press(&mut self, _ks: &super::KeyState) -> EVec {
        if self.hold_while_undecided {
            evec![Event::bkey_down(self.hold.clone().into())]
        } else {
            evec![Event::None]
        }
    }

    fn on_release(&mut self, _ks: &super::KeyState) -> EVec {
        // TODO this is broken because i can't pass state via Behaviors (they are passed around too
        // freely). I may want to use the KeyState argument to pass in the held keys here.

        // Behavior key is released, so we want to "unpress" whatever key has been sent
        if self.decided_tap {
            panic!("Shouldn't happen currently (until support for bilateral combinations is added)")
        } else if self.decided_hold {
            // decided_hold set means delay expired and after_delay fired. Release hold now
            evec![Event::bkey_up(self.hold.clone().into())]
        } else {
            // Released before timeout, is tap
            self.decided_tap = true;

            if self.hold_while_undecided {
                // Release hold and send special tap
                evec![
                    Event::bkey_up(self.hold.clone().into()),
                    Event::special_tap(self.tap.clone().into())
                ]
            } else {
                evec![Event::special_tap(self.tap.clone().into())]
            }
        }
    }

    fn try_get_delay(&self) -> Option<crate::timer::Duration> {
        Some(self.duration)
    }

    fn after_delay(&mut self, _ks: &super::KeyState) -> EVec {
        if self.decided_hold || self.decided_tap {
            // If event is already decided there's nothing we need to do
            evec![]
        } else {
            self.decided_hold = true;

            if self.hold_while_undecided {
                // If hold_while_undecided is set, hold key event is already sent
                evec![]
            } else {
                evec![Event::bkey_down(self.hold.clone().into())]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        behavior::{KeyState, key_press::KeyPress},
        key::Key,
    };

    use super::*;

    #[test]
    fn test_hold_tap() {
        let kp_t = KeyPress::new(Key::T);
        let kp_h = KeyPress::new(Key::H);

        let mut ht1 = HoldTap {
            hold: kp_h.into(),
            tap: kp_t.into(),
            hold_while_undecided: true,
            decided_tap: false,
            decided_hold: false,
            duration: Duration::new(0),
        };
        let mut ht2 = HoldTap {
            hold: kp_h.into(),
            tap: kp_t.into(),
            hold_while_undecided: true,
            decided_tap: false,
            decided_hold: false,
            duration: Duration::new(0),
        };

        // Test timeout expired
        assert_eq!(
            ht1.on_press(&KeyState {}),
            evec![Event::bkey_down(kp_h.into())]
        );
        assert_eq!(ht1.after_delay(&KeyState {}), evec![]);
        assert_eq!(
            ht1.on_release(&KeyState {}),
            evec![Event::bkey_up(kp_h.into())]
        );
        assert_eq!(ht1.try_get_delay(), Some(Duration::new(0)));
        assert!(ht1.decided_hold);
        assert!(!ht1.decided_tap);

        assert_eq!(
            ht2.on_press(&KeyState {}),
            evec![Event::bkey_down(kp_h.into())]
        );
        assert_eq!(
            ht2.on_release(&KeyState {}),
            evec![
                Event::bkey_up(kp_h.into()),
                Event::special_tap(KeyPress::new(Key::T).into())
            ]
        );
        assert_eq!(ht2.after_delay(&KeyState {}), evec![]);
        assert_eq!(ht2.try_get_delay(), Some(Duration::new(0)));
        assert!(!ht2.decided_hold);
        assert!(ht2.decided_tap);
    }
}
