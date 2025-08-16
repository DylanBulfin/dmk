use crate::{
    behavior::Behavior,
    evec,
    event::{EVec, Event, KeyEvent},
    key::Key,
    timer::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyPress {
    key: Key,
}

impl KeyPress {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

impl Behavior for KeyPress {
    fn on_press(&mut self, _ks: &super::KeyState) -> EVec {
        evec![Event::key_down(self.key)]
    }

    fn on_release(&mut self, _ks: &super::KeyState) -> EVec {
        evec![Event::key_up(self.key)]
    }

    fn try_get_delay(&self) -> Option<Duration> {
        None
    }

    fn after_delay(&mut self, _ks: &super::KeyState) -> EVec {
        evec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_press() {
        let mut kp = KeyPress { key: Key::A };

        assert_eq!(
            kp.on_press(&crate::behavior::KeyState {}),
            evec![Event::key_down(Key::A)]
        );
        assert_eq!(
            kp.on_release(&crate::behavior::KeyState {}),
            evec![Event::key_up(Key::A)]
        );
        assert_eq!(kp.try_get_delay(), None);
        assert_eq!(kp.after_delay(&crate::behavior::KeyState {}), evec![]);
    }
}
