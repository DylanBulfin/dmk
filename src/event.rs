use core::{
    mem,
    ops::{Index, IndexMut},
};

use crate::{
    behavior::{Behavior, DefaultBehavior},
    key::Key,
};

pub mod queue;

pub const EVEC_LEN: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct EVec {
    arr: [Event; EVEC_LEN],
    len: usize,
}

impl EVec {
    pub fn new() -> Self {
        Self {
            arr: [Event::None; EVEC_LEN],
            len: 0,
        }
    }

    pub fn push(&mut self, event: Event) {
        if self.len < EVEC_LEN {
            self.arr[self.len] = event;
            self.len += 1;
        } else {
            panic!("Trying to add event to EVec at max size")
        }
    }

    /// Pops a value from the *back* of the EVec, returns None if empty
    pub fn pop(&mut self) -> Option<Event> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            let res = self.arr[self.len];
            self.arr[self.len] = Event::None;

            Some(res)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Index<usize> for EVec {
    type Output = Event;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Attempted to access past end of array")
        } else {
            &self.arr[index]
        }
    }
}

impl IndexMut<usize> for EVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Attempted to access past end of array")
        } else {
            &mut self.arr[index]
        }
    }
}

impl IntoIterator for EVec {
    type Item = Event;
    type IntoIter = EVecIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            base: self,
            cursor: 0,
        }
    }
}

pub struct EVecIter {
    base: EVec,
    cursor: usize,
}

impl Iterator for EVecIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.base.len {
            let res = mem::replace(&mut self.base[self.cursor], Event::None);
            self.cursor += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! evec {
    [$($elem:expr),*] => {{
        let mut base = crate::event::EVec::new();
        for elem in [$($elem),*]{
            base.push(elem);
        }

        base
    }};
}

/// These are the kind of special events that can happen. The initial usecase is to allow key
/// presses to send taps specifically. For example, if a hold-tap on_release method is called and
/// the timeout has not expired, the on_release method needs to trigger a full tap of a key rather
/// than individual KeyUp/KeyDown events
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecialEvent {
    TapBehavior(DefaultBehavior),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Corresponds to either a physical button press on the keyboard or the output of another
    /// behavior (e.g. a layer/tap would generate a momentary layer switch)
    BehaviorKeyEvent(BehaviorKeyEvent),
    /// Corresponds to a keypress output of a behavior (such as the keypress behavior)
    KeyEvent(KeyEvent),
    LayerEvent(LayerEvent),
    SpecialEvent(SpecialEvent),
    None,
}

impl Event {
    pub fn bkey_up(behavior: DefaultBehavior) -> Self {
        Self::BehaviorKeyEvent(BehaviorKeyEvent {
            behavior,
            is_press: false,
        })
    }

    pub fn bkey_down(behavior: DefaultBehavior) -> Self {
        Self::BehaviorKeyEvent(BehaviorKeyEvent {
            behavior,
            is_press: true,
        })
    }

    pub fn key_up(key: Key) -> Self {
        Self::KeyEvent(KeyEvent {
            key,
            is_press: false,
        })
    }

    pub fn key_down(key: Key) -> Self {
        Self::KeyEvent(KeyEvent {
            key,
            is_press: true,
        })
    }

    pub fn special_tap(behavior: DefaultBehavior) -> Self {
        Self::SpecialEvent(SpecialEvent::TapBehavior(behavior))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyEvent {
    pub key: Key,
    pub is_press: bool,
}

impl From<KeyEvent> for Event {
    fn from(value: KeyEvent) -> Self {
        Self::KeyEvent(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BehaviorKeyEvent {
    pub behavior: DefaultBehavior,
    pub is_press: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerEvent {
    AddLayer(usize),
    RemoveDownToLayer(usize),

}

impl From<LayerEvent> for Event {
    fn from(value: LayerEvent) -> Self {
        Self::LayerEvent(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{Event, Key, KeyEvent};

    #[test]
    fn test_evec_macro() {
        let a = evec![Event::key_up(Key::A)];

        assert_eq!(a[0], Event::key_up(Key::A));
        for i in 1..a.len() {
            assert_eq!(a[i], Event::None);
        }
    }
}
