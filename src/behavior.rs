use core::cell::Cell;

use crate::{
    event::{Event, KeyEvent, LayerEvent, SimpleBehaviorEvent, SimpleKeyEvent}, layer::Layer, timer::Duration, vboard::Key
};

pub trait BehaviorSimple {
    fn on_activate(&self) -> Option<Event>;
    fn on_deactivate(&self) -> Option<Event>;
}
pub trait BehaviorComplex {
    fn on_press(&mut self) -> Option<Event>;
    fn on_unpress(&mut self) -> Option<Event>;
    fn get_duration(&self) -> Option<Duration>;
    fn on_timeout(&mut self) -> Option<Event>;
}

#[derive(Debug, Clone, Copy)]
/// A simble behavior, one that does not require any specialized logic like timer events. Can be
/// used as an "arument" for other behaviors
pub enum SimpleBehavior {
    KeyPress(KeyPressBehavior),
    MomentaryLayer(MomentaryLayerBehavior),
}
impl BehaviorSimple for SimpleBehavior {
    fn on_activate(&self) -> Option<Event> {
        match self {
            SimpleBehavior::KeyPress(b) => b.on_activate(),
            SimpleBehavior::MomentaryLayer(b) => b.on_activate(),
        }
    }

    fn on_deactivate(&self) -> Option<Event> {
        match self {
            SimpleBehavior::KeyPress(b) => b.on_deactivate(),
            SimpleBehavior::MomentaryLayer(b) => b.on_deactivate(),
        }
    }
}
// SimpleBehaviors can also be used in a manual context like any other, and need to implement the
// trait
impl BehaviorComplex for SimpleBehavior {
    fn on_press(&mut self) -> Option<Event> {
        self.on_activate()
    }

    fn on_unpress(&mut self) -> Option<Event> {
        self.on_deactivate()
    }

    fn get_duration(&self) -> Option<Duration> {
        None
    }

    fn on_timeout(&mut self) -> Option<Event> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyPressBehavior(Key);
impl BehaviorSimple for KeyPressBehavior {
    fn on_activate(&self) -> Option<Event> {
        Some(Event::KeyEvent(KeyEvent::Simple(SimpleKeyEvent::Press(
            self.0,
        ))))
    }

    fn on_deactivate(&self) -> Option<Event> {
        Some(Event::KeyEvent(KeyEvent::Simple(SimpleKeyEvent::Unpress(
            self.0,
        ))))
    }
}

#[derive(Debug, Clone, Copy)]
/// Holds both "from" layer and "to" layer because when released, it needs to pop the layer
/// stack down to the "from" layer.
pub struct MomentaryLayerBehavior(Layer, Layer);
impl BehaviorSimple for MomentaryLayerBehavior {
    fn on_activate(&self) -> Option<Event> {
        Some(Event::LayerEvent(LayerEvent::AddLayer(self.1)))
    }

    fn on_deactivate(&self) -> Option<Event> {
        Some(Event::LayerEvent(LayerEvent::RemoveToLayer(self.0)))
    }
}

#[derive(Debug)]
pub enum ManualBehavior {
    HoldTap(HoldTapBehavior),
    Simple(SimpleBehavior),
}

impl BehaviorComplex for ManualBehavior {
    fn on_press(&mut self) -> Option<Event> {
        match self {
            ManualBehavior::HoldTap(b) => b.on_press(),
            ManualBehavior::Simple(b) => b.on_press(),
        }
    }

    fn on_unpress(&mut self) -> Option<Event> {
        match self {
            ManualBehavior::HoldTap(b) => b.on_unpress(),
            ManualBehavior::Simple(b) => b.on_unpress(),
        }
    }

    fn get_duration(&self) -> Option<Duration> {
        match self {
            ManualBehavior::HoldTap(b) => b.get_duration(),
            ManualBehavior::Simple(b) => b.get_duration(),
        }
    }

    fn on_timeout(&mut self) -> Option<Event> {
        match self {
            ManualBehavior::HoldTap(b) => b.on_timeout(),
            ManualBehavior::Simple(b) => b.on_timeout(),
        }
    }
}

#[derive(Debug)]
pub enum HoldTapBehaviorState {
    Pending,
    DecidedTap,
    DecidedHold,
}
#[derive(Debug)]
pub struct HoldTapBehavior {
    state: HoldTapBehaviorState,
    hold: SimpleBehavior,
    tap: SimpleBehavior,
    timeout: Duration,
    hold_while_undecided: bool,
}
impl BehaviorComplex for HoldTapBehavior {
    fn on_press(&mut self) -> Option<Event> {
        if self.hold_while_undecided {
            Some(Event::BehaviorEvent(SimpleBehaviorEvent::StartBehavior(
                self.hold,
            )))
        } else {
            None
        }
    }

    fn on_unpress(&mut self) -> Option<Event> {
        match self.state {
            HoldTapBehaviorState::DecidedTap => {
                panic!("Invalid state: DecidedTap encountered in on_deactivate")
            }
            HoldTapBehaviorState::DecidedHold => Some(Event::BehaviorEvent(
                SimpleBehaviorEvent::EndBehavior(self.hold),
            )),
            HoldTapBehaviorState::Pending => {
                self.state = HoldTapBehaviorState::DecidedTap;
                Some(Event::BehaviorEvent(SimpleBehaviorEvent::TapBehavior(
                    self.tap,
                )))
            }
        }
    }

    fn get_duration(&self) -> Option<Duration> {
        Some(self.timeout)
    }

    fn on_timeout(&mut self) -> Option<Event> {
        match self.state {
            HoldTapBehaviorState::Pending => {
                // Decide as hold
                self.state = HoldTapBehaviorState::DecidedHold;
                if self.hold_while_undecided {
                    None
                } else {
                    Some(Event::BehaviorEvent(SimpleBehaviorEvent::StartBehavior(
                        self.hold,
                    )))
                }
            }
            _ => None,
        }
    }
}

const BEHAVIOR_ID: Cell<usize> = Cell::new(0);
pub fn get_behavior_id() -> usize {
    let val = BEHAVIOR_ID.get();
    BEHAVIOR_ID.set(val + 1);
    val
}
