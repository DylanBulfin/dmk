use crate::{
    behavior::Behavior,
    evec,
    event::{EVec, Key, KeyEvent},
    timer::Duration,
};

pub struct KeyPressBehavior {
    key: Key,
}

impl Behavior for KeyPressBehavior {
    fn on_press(&mut self, _ks: &super::KeyState) -> EVec {
        evec![KeyEvent::KeyDown(self.key).into()]
    }

    fn on_release(&mut self, _ks: &super::KeyState) -> EVec {
        evec![KeyEvent::KeyUp(self.key).into()]
    }

    fn try_get_delay(&self) -> Option<Duration> {
        None
    }

    fn after_delay(&mut self, _ks: &super::KeyState) -> EVec {
        evec![]
    }
}
