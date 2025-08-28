use crate::{behavior::SimpleBehavior, layer::Layer, vboard::Key};

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
