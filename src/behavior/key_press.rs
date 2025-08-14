use crate::behavior::{Behavior, Key, KeyEvent};

pub struct KeyPressBehavior {
    key: Key,
    state: KeyPressBehaviorState,
}

enum KeyPressBehaviorState {
    Init,
    Pressed,
    Done,
}

impl Behavior for KeyPressBehavior {
    fn call<I>(&mut self, ks: &super::KeyState) -> super::BehaviorResult<I>
    where
        I: Iterator<Item = KeyEvent>,
    {
        match self.state {
            KeyPressBehaviorState::Init => ,
            KeyPressBehaviorState::Pressed => todo!(),
            KeyPressBehaviorState::Done => todo!(),
        }
    }
}
