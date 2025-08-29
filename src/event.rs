use core::cmp::Ordering;

use crate::{behavior::SimpleBehavior, layer::Layer, timer::Instant, vboard::Key};

pub enum Event {
    KeyEvent(KeyEvent),
    BehaviorEvent(SimpleBehaviorEvent),
    LayerEvent(LayerEvent),
}
pub enum ComplexKeyEvent {
    ReleaseTap(Key, Key),
    ReleasePress(Key, Key),
}
pub enum SimpleKeyEvent {
    Press(Key),
    Unpress(Key),
}
pub enum KeyEvent {
    Complex(ComplexKeyEvent),
    Simple(SimpleKeyEvent),
}

pub enum LayerEvent {
    AddLayer(Layer),
    RemoveToLayer(Layer),
}

pub enum SimpleBehaviorEvent {
    StartBehavior(SimpleBehavior),
    EndBehavior(SimpleBehavior),
    TapBehavior(SimpleBehavior),
    ReleasePressBehavior(SimpleBehavior, SimpleBehavior), // Release the first and press the second
    ReleaseTapBehavior(SimpleBehavior, SimpleBehavior),   // Release the first and tap the second
}

pub enum TimerEvent {
    Behavior(BehaviorTimeoutEvent),
}

impl PartialEq for TimerEvent {
    fn eq(&self, other: &Self) -> bool {
        let lhs_inst = match self {
            TimerEvent::Behavior(e) => e.instant,
        };
        let rhs_inst = match other {
            TimerEvent::Behavior(e) => e.instant,
        };

        lhs_inst == rhs_inst
    }
}

impl Eq for TimerEvent {}

impl PartialOrd for TimerEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs_inst = match self {
            TimerEvent::Behavior(e) => e.instant,
        };
        let rhs_inst = match other {
            TimerEvent::Behavior(e) => e.instant,
        };

        lhs_inst.cmp(&rhs_inst)
    }
}

pub struct BehaviorTimeoutEvent {
    behavior_id: usize,
    instant: Instant,
}

impl PartialEq for BehaviorTimeoutEvent {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}
