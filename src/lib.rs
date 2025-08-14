use std::marker::PhantomData;

pub struct KeyState {}
pub enum Key {}
pub struct Timer {}

pub trait Behavior {
    type Func;
    type Arg;

    fn call(&mut self, ks: &KeyState) -> BehaviorResult;
}

pub struct ImmediateBehavior<F, A>
where
    F: FnOnce(&KeyState, A) -> Vec<Key>,
{
    f: Option<F>,
    a: A,
}

impl<F, A> Behavior for ImmediateBehavior<F, A>
where
    F: FnOnce(&KeyState, A) -> Vec<Key>,
{
    type Func = F;
    type Arg = A;

    fn call(&mut self, ks: &KeyState) -> BehaviorResult {
        BehaviorResult {
            keys: (self
                .f
                .take()
                .expect("Call on already-used ImmediateBehavior"))(k, a),
            timer: None,
        }
    }
}

pub struct DelayedBehavior<F, A>
where
    F: FnOnce(&KeyState, A) -> Vec<Key>,
{
    state: DelayedBehaviorState,
    init: Option<F>,
    timer: Option<Timer>,
    callback: Option<F>,
    a: A,
}

// impl<F, A> DelayedBehavior<F, A> where
//     F: FnOnce(&KeyState, A) -> Vec<Key>,{
//         pub fn new(f: F)
//     }
//
#[derive(Debug, PartialEq, Eq)]
enum DelayedBehaviorState {
    Init,
    Delaying,
    Finished,
}

impl<F, A> Behavior for DelayedBehavior<F, A>
where
    F: FnOnce(&KeyState, A) -> Vec<Key>,
{
    type Func = F;
    type Arg = A;

    fn call(&mut self, ks: &KeyState) -> BehaviorResult {
        match self.state {
            DelayedBehaviorState::Init => BehaviorResult {
                keys: self.init.take().map(|i| i(ks, self.a)).unwrap_or_default(),
                timer: self.timer.take(),
            },
            DelayedBehaviorState::Delaying => BehaviorResult {
                keys: (self
                    .callback
                    .take()
                    .expect("State mismatch: state = Delaying, but callback is None"))(
                    ks, self.a
                ),
                timer: None,
            },
            DelayedBehaviorState::Finished => {
                panic!("Unexpected call on finished DelayedBehavior")
            }
        }
    }
}

pub struct BehaviorResult {
    pub keys: Vec<Key>,
    pub timer: Option<Timer>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
