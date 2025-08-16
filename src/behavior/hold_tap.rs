//! Hold-tap, a fundamental behavior in any keyboard firmware

use crate::{
    behavior::{Behavior, DefaultBehavior, key_press::KeyPress},
    evec,
    event::{EVec, Event},
    key::Key,
    timer::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoldTap {
    decided_hold: bool,
    decided_tap: bool,
    hold: &'b DefaultBehavior,
    tap: Key,
    hold_while_undecided: bool,
    duration: Duration,
}

impl Behavior for HoldTap {
    fn on_press(&mut self, _ks: &super::KeyState) -> EVec {
        if self.hold_while_undecided {
            evec![Event::key_down(self.hold)]
        } else {
            evec![Event::None]
        }
    }

    fn on_release(&mut self, _ks: &super::KeyState) -> EVec {
        // Behavior key is released, so we want to "unpress" whatever key has been sent
        if self.decided_tap {
            panic!("Shouldn't happen currently (until support for bilateral combinations is added)")
        } else if self.decided_hold {
            // decided_hold set means delay expired and after_delay fired. Release hold now
            evec![Event::key_up(self.hold)]
        } else {
            // Released before timeout, is tap
            self.decided_tap = true;

            if self.hold_while_undecided {
                // Release hold and send special tap
                evec![
                    Event::key_up(self.hold),
                    Event::special_tap(KeyPress::new(self.tap).into())
                ]
            } else {
                evec![Event::special_tap(KeyPress::new(self.tap).into())]
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
                evec![Event::key_down(self.hold)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::behavior::KeyState;

    use super::*;

    #[test]
    fn test_hold_tap() {
        let mut ht1 = HoldTap {
            hold: Key::H,
            tap: Key::T,
            hold_while_undecided: true,
            decided_tap: false,
            decided_hold: false,
            duration: Duration::new(0),
        };
        let mut ht2 = HoldTap {
            hold: Key::H,
            tap: Key::T,
            hold_while_undecided: true,
            decided_tap: false,
            decided_hold: false,
            duration: Duration::new(0),
        };

        // Test timeout expired
        assert_eq!(ht1.on_press(&KeyState {}), evec![Event::key_down(Key::H)]);
        assert_eq!(ht1.after_delay(&KeyState {}), evec![]);
        assert_eq!(ht1.on_release(&KeyState {}), evec![Event::key_up(Key::H)]);
        assert_eq!(ht1.try_get_delay(), Some(Duration::new(0)));
        assert!(ht1.decided_hold);
        assert!(!ht1.decided_tap);

        assert_eq!(ht2.on_press(&KeyState {}), evec![Event::key_down(Key::H)]);
        assert_eq!(
            ht2.on_release(&KeyState {}),
            evec![
                Event::key_up(Key::H),
                Event::special_tap(KeyPress::new(Key::T).into())
            ]
        );
        assert_eq!(ht2.after_delay(&KeyState {}), evec![]);
        assert_eq!(ht2.try_get_delay(), Some(Duration::new(0)));
        assert!(!ht2.decided_hold);
        assert!(ht2.decided_tap);
    }
}
