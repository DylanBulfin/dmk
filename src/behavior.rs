pub mod key_press;

pub struct KeyState {}

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyEvent {
    KeyUp(Key),
    KeyDown(Key),
    None,
}

#[derive(Clone, Debug)]
pub struct Duration {}

pub trait Behavior<I>
where
    I: Iterator<Item = KeyEvent>,
{
    fn call(&mut self, ks: &KeyState) -> BehaviorResult<I>;
}

pub struct ImmediateBehavior<F, D, I>
where
    F: FnOnce(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    callback: Option<F>,
    data: D,
}

impl<F, D, I> ImmediateBehavior<F, D, I>
where
    F: FnOnce(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    pub fn new(callback: F, data: D) -> Self {
        Self {
            callback: Some(callback),
            data,
        }
    }
}

impl<F, D, I> Behavior<I> for ImmediateBehavior<F, D, I>
where
    F: FnOnce(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    fn call(&mut self, ks: &KeyState) -> BehaviorResult<I> {
        match self.callback.take() {
            Some(f) => BehaviorResult {
                keys: f(&mut self.data, ks),
                duration: None,
            },
            _ => panic!("ImmediateBehavior function is None"),
        }
    }
}

pub struct DelayedBehavior<FI, FC, I, D>
where
    FI: FnOnce(&mut D, &KeyState) -> I,
    FC: FnOnce(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    init: Option<FI>,
    duration: Option<Duration>,
    callback: Option<FC>,
    data: D,
}

impl<FI, FC, I, D> DelayedBehavior<FI, FC, I, D>
where
    FI: FnOnce(&mut D, &KeyState) -> I,
    FC: FnOnce(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    pub fn new(init: FI, timer: Duration, callback: FC, data: D) -> Self {
        Self {
            init: Some(init),
            duration: Some(timer),
            callback: Some(callback),
            data,
        }
    }
}

impl<FI, FC, I, D> Behavior<I> for DelayedBehavior<FI, FC, I, D>
where
    FI: FnOnce(&mut D, &KeyState) -> I,
    FC: FnOnce(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    fn call(&mut self, ks: &KeyState) -> BehaviorResult<I> {
        if let Some(f) = self.init.take() {
            // Initial call
            BehaviorResult {
                keys: f(&mut self.data, ks),
                duration: self.duration.take(),
            }
        } else if let Some(f) = self.callback.take() {
            // Delayed call
            BehaviorResult {
                keys: f(&mut self.data, ks),
                duration: None,
            }
        } else {
            panic!("Both init and callback functions are None for DelayedBehavior")
        }
    }
}

pub struct CyclicBehavior<F, D, I>
where
    F: FnMut(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    callback: F,
    duration: Duration,
    data: D,
}

impl<F, D, I> CyclicBehavior<F, D, I>
where
    F: FnMut(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    pub fn new(callback: F, timer: Duration, data: D) -> Self {
        Self {
            callback,
            duration: timer,
            data,
        }
    }
}

impl<F, D, I> Behavior<I> for CyclicBehavior<F, D, I>
where
    F: FnMut(&mut D, &KeyState) -> I,
    I: Iterator<Item = KeyEvent>,
{
    fn call(&mut self, ks: &KeyState) -> BehaviorResult<I> {
        BehaviorResult {
            keys: (self.callback)(&mut self.data, ks),
            duration: Some(self.duration.clone()),
        }
    }
}

pub struct BehaviorResult<I>
where
    I: Iterator<Item = KeyEvent>,
{
    pub keys: I,
    pub duration: Option<Duration>,
}

// These mostly just test that you can sensibly create and use behaviors
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_behavior() {
        let f = |d: &mut char, _ks: &KeyState| {
            *d = 'A';
            [KeyEvent::KeyUp(Key::A)].into_iter()
        };
        let mut ib = ImmediateBehavior::new(f, '1');

        let BehaviorResult {
            mut keys,
            duration: timer,
        } = ib.call(&KeyState {});

        assert_eq!(keys.next(), Some(KeyEvent::KeyUp(Key::A)));
        assert_eq!(keys.next(), None);
        assert!(timer.is_none());
    }

    #[test]
    #[should_panic(expected = "ImmediateBehavior function is None")]
    fn test_call_immediate_twice_panic() {
        let f = |d: &mut char, _ks: &KeyState| {
            *d = 'A';
            [KeyEvent::KeyUp(Key::A)].into_iter()
        };
        let mut ib = ImmediateBehavior::new(f, '1');

        let _ = ib.call(&KeyState {});
        let _ = ib.call(&KeyState {});
    }

    #[test]
    fn test_delayed_behavior() {
        let init = |d: &mut char, _ks: &KeyState| {
            *d = 'A';
            [KeyEvent::KeyUp(Key::A)].into_iter()
        };
        let callback = |d: &mut char, _ks: &KeyState| {
            *d = 'B';
            [KeyEvent::KeyUp(Key::B)].into_iter()
        };
        let mut db = DelayedBehavior::new(init, Duration {}, callback, '1');

        // Test call of init
        let BehaviorResult {
            mut keys,
            duration: timer,
        } = db.call(&KeyState {});

        assert_eq!(keys.next(), Some(KeyEvent::KeyUp(Key::A)));
        assert_eq!(keys.next(), None);
        assert!(timer.is_some());
        assert!(db.init.is_none());
        assert_eq!(db.data, 'A');

        // Test call of main callback
        let BehaviorResult {
            mut keys,
            duration: timer,
        } = db.call(&KeyState {});
        assert_eq!(keys.next(), Some(KeyEvent::KeyUp(Key::B)));
        assert_eq!(keys.next(), None);
        assert!(timer.is_none());
        assert!(db.init.is_none());
        assert!(db.callback.is_none());
        assert!(db.duration.is_none());
        assert_eq!(db.data, 'B');
    }

    #[test]
    #[should_panic(expected = "Both init and callback functions are None for DelayedBehavior")]
    fn test_call_delayed_thrice_panic() {
        let init = |d: &mut char, _ks: &KeyState| {
            *d = 'A';
            [KeyEvent::KeyUp(Key::A)].into_iter()
        };
        let callback = |d: &mut char, _ks: &KeyState| {
            *d = 'A';
            [KeyEvent::KeyUp(Key::A)].into_iter()
        };
        let mut db = DelayedBehavior::new(init, Duration {}, callback, '1');

        let _ = db.call(&KeyState {});
        let _ = db.call(&KeyState {});
        let _ = db.call(&KeyState {});
    }

    #[test]
    fn test_cyclic_behavior() {
        let callback = |d: &mut char, _ks: &KeyState| {
            *d = 'A';
            [KeyEvent::KeyUp(Key::A)].into_iter()
        };
        let mut cb = CyclicBehavior::new(callback, Duration {}, '1');

        for _ in 1..=10 {
            let BehaviorResult {
                mut keys,
                duration: timer,
            } = cb.call(&KeyState {});
            assert_eq!(keys.next(), Some(KeyEvent::KeyUp(Key::A)));
            assert_eq!(keys.next(), None);
            assert!(timer.is_some());
            assert_eq!(cb.data, 'A');
        }
    }
}
