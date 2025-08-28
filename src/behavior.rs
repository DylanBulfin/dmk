use core::cell::Cell;

#[derive(Debug, Clone, Copy)]
pub struct Layer {}
#[derive(Debug, Clone, Copy)]
pub struct Key {}
pub struct Duration {}
pub struct Event {}
pub enum ComplexKeyEvent {
    ReleaseTap(Key, Key),
    ReleasePress(Key, Key),
}
pub enum KeyEvent {
    Complex(ComplexKeyEvent),
    Simple,
}

#[derive(Debug, Clone, Copy)]
/// A simble behavior, one that does not require any specialized logic like timer events. Can be
/// used as an "arument" for other behaviors
pub enum SimpleBehavior {
    KeyPress(Key),
    /// Holds both "from" layer and "to" layer because when released, it needs to pop the layer
    /// stack down to the "from" layer.
    MomentaryLayer(Layer, Layer),
}

impl SimpleBehavior {
    pub fn on_activate(&self) -> Event {
        unimplemented!()
    }
    pub fn on_deactivate(&self) -> Event {
        unimplemented!()
    }
}

pub enum ComplexBehavior {
    HoldTap(HoldTapBehavior),
}

pub enum HoldTapBehaviorState {
    Pending,
    DecidedTap,
    DecidedHold,
}

pub struct HoldTapBehavior {
    state: HoldTapBehaviorState,
    hold: SimpleBehavior,
    tap: SimpleBehavior,
    timeout: Duration,
    hold_while_undecided: bool,
}

const BEHAVIOR_ID: Cell<usize> = Cell::new(0);
pub fn get_behavior_id() -> usize {
    let val = BEHAVIOR_ID.get();
    BEHAVIOR_ID.set(val + 1);
    val
}
