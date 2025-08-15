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

/// These are the kind of special events that can happen. The initial usecase is to allow key
/// presses to send taps specifically. For example, if retro-tap is on the hold_tap behavior needs
/// to be able to trigger a tap with a single event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpecialEventKind {
    Tap(Key),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    KeyEvent(KeyEvent),
    LayerEvent(LayerEvent),
    SpecialEvent(SpecialEventKind),
    None,
}

impl Event {
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    // Alphas
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,

    // Mods
    LALT,
    RALT,
    LGUI,
    RGUI,
    LCTL,
    RCTL,
    LSFT,
    RSFT,

    // Directions
    UP,
    DOWN,
    LEFT,
    RIGHT,

    // Function Keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // Nav Keys
    HOME,
    END,
    PGDN,
    PGUP,

    // Symbols
    DOT,
    COMMA,
    BTICK,
    FSLASH,
    BSLASH,
    DASH,
    EQUAL,
    LBRACK,
    RBRACK,

    // Control
    SPACE,
    ENTER,
    BSPACE,
    DEL,
    ESC,
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
pub struct LayerEvent {}

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
