use std::marker::PhantomData;

pub struct KeyState {}
pub enum Key {}
pub struct Timer {}

pub trait Behavior {
    type Func;

    fn call(&mut self, ks: &KeyState) -> BehaviorResult;
}

pub struct ImmediateBehavior<F>
where
    F: FnOnce(&KeyState) -> Vec<Key>,
{
    f: Option<F>,
}

impl<F> Behavior for ImmediateBehavior<F>
where
    F: FnOnce(&KeyState) -> Vec<Key>,
{
    type Func = F;

    fn call(&mut self, ks: &KeyState) -> BehaviorResult {
        BehaviorResult {
            keys: (self
                .f
                .take()
                .expect("Call on already-used ImmediateBehavior"))(ks),
            timer: None,
        }
    }
}

pub struct DelayedBehavior<F>
where
    F: FnOnce(&KeyState) -> Vec<Key>,
{
    state: DelayedBehaviorState,
    init: Option<F>,
    timer: Option<Timer>,
    callback: Option<F>,
}

impl<F> DelayedBehavior<F>
where
    F: FnOnce(&KeyState) -> Vec<Key>,
{
    pub fn new(init: F, timer: Timer, callback: F) -> Self {
        Self {
            state: DelayedBehaviorState::Init,
            init: Some(init),
            timer: Some(timer),
            callback: Some(callback),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DelayedBehaviorState {
    Init,
    Delaying,
    Finished,
}

impl<F> Behavior for DelayedBehavior<F>
where
    F: FnOnce(&KeyState) -> Vec<Key>,
{
    type Func = F;

    fn call(&mut self, ks: &KeyState) -> BehaviorResult {
        match self.state {
            DelayedBehaviorState::Init => {
                self.state = DelayedBehaviorState::Delaying;
                BehaviorResult {
                    keys: self.init.take().map(|i| i(ks)).unwrap_or_default(),
                    timer: self.timer.take(),
                }
            }
            DelayedBehaviorState::Delaying => {
                self.state = DelayedBehaviorState::Finished;
                BehaviorResult {
                    keys: (self
                        .callback
                        .take()
                        .expect("State mismatch: state = Delaying, but callback is None"))(
                        ks
                    ),
                    timer: None,
                }
            }
            DelayedBehaviorState::Finished => {
                panic!("Unexpected call on finished DelayedBehavior")
            }
        }
    }
}

pub struct CyclicBehavior

pub struct BehaviorResult {
    pub keys: Vec<Key>,
    pub timer: Option<Timer>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
