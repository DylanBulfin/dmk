pub const EVEC_LEN: usize = 5;

/// This type represents the return type of a behavior
pub type EVec = [Event; EVEC_LEN];

#[macro_export]
macro_rules! evec {
    [$($elem:expr),*] => {{
        let mut base = [crate::event::Event::None; crate::event::EVEC_LEN];
        for (i, elem) in [$($elem),*].into_iter().enumerate() {
            base[i] = elem;
        }

        base
    }};
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    KeyEvent(KeyEvent),
    LayerEvent(LayerEvent),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyEvent {
    KeyUp(Key),
    KeyDown(Key),
}

impl From<KeyEvent> for Event {
    fn from(value: KeyEvent) -> Self {
        Self::KeyEvent(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayerEvent {}

impl From<LayerEvent> for Event {
    fn from(value: LayerEvent) -> Self {
        Self::LayerEvent(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{Event, KeyEvent};

    #[test]
    fn test_evec_macro() {
        let a = evec![Event::KeyEvent(crate::event::KeyEvent::KeyUp(
            crate::event::Key::A
        ))];

        assert_eq!(a[0], KeyEvent::KeyUp(crate::event::Key::A).into());
        for i in 1..a.len() {
            assert_eq!(a[i], Event::None);
        }
    }
}
