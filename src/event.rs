use core::cmp::Ordering;

use crate::{behavior::SimpleBehavior, layer::Layer, timer::Instant, vboard::Key};

pub struct Event {
    pub behavior_id: usize,
    pub data: EventData,
}

impl Event {
    pub fn new(behavior_id: usize, data: EventData) -> Self {
        Self { behavior_id, data }
    }
}

pub enum EventData {
    KeyEvent(KeyEvent),
    BehaviorEvent(BehaviorEvent),
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

pub enum BehaviorEvent {
    StartBehavior(SimpleBehavior),
    EndBehavior(SimpleBehavior),
    TapBehavior(SimpleBehavior), // Taps are performed by main processing loop, no need to
    ReleasePressBehavior(SimpleBehavior, SimpleBehavior), // Release the first and press the second
    ReleaseTapBehavior(SimpleBehavior, SimpleBehavior),   // Release the first and tap the second
}
