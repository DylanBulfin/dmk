use crate::{
    behavior::Behavior,
    evec,
    event::{EVec, Event, Key, KeyEvent},
    timer::Duration,
};

pub struct KeyPressBehavior {
    key: Key,
}

impl Behavior for KeyPressBehavior {
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
