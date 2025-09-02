use core::cell::Cell;

use crate::{
    event::{BehaviorEvent, Event, EventData, KeyEvent, LayerEvent, SimpleKeyEvent},
    layer::Layer,
    timer::Duration,
    vboard::Key,
};

pub trait BehaviorSimple {
    fn on_activate(&self) -> Option<Event>;
    fn on_deactivate(&self) -> Option<Event>;
    fn behavior_id(&self) -> usize;
}
pub trait BehaviorComplex {
    fn on_press(&mut self) -> Option<Event>;
    fn on_unpress(&mut self) -> Option<Event>;
    fn get_duration(&self) -> Option<Duration>;
    fn on_timeout(&mut self) -> Option<Event>;
    fn id(&self) -> usize;
}

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

    fn id(&self) -> usize {
        self.behavior_id()
    }
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

    fn behavior_id(&self) -> usize {
        match self {
            SimpleBehavior::KeyPress(e) => e.behavior_id(),
            SimpleBehavior::MomentaryLayer(e) => e.behavior_id(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyPressBehavior {
    key: Key,
    behavior_id: usize,
}

impl BehaviorSimple for KeyPressBehavior {
    fn on_activate(&self) -> Option<Event> {
        Some(Event::new(
            self.behavior_id,
            EventData::KeyEvent(KeyEvent::Simple(SimpleKeyEvent::Press(self.key))),
        ))
    }

    fn on_deactivate(&self) -> Option<Event> {
        Some(Event::new(
            self.behavior_id,
            EventData::KeyEvent(KeyEvent::Simple(SimpleKeyEvent::Unpress(self.key))),
        ))
    }

    fn behavior_id(&self) -> usize {
        self.behavior_id
    }
}

#[derive(Debug, Clone, Copy)]
/// Holds both "from" layer and "to" layer because when released, it needs to pop the layer
/// stack down to the "from" layer.
pub struct MomentaryLayerBehavior {
    layer_from: Layer,
    layer_to: Layer,
    behavior_id: usize,
}
impl BehaviorSimple for MomentaryLayerBehavior {
    fn on_activate(&self) -> Option<Event> {
        Some(Event::new(
            self.behavior_id,
            EventData::LayerEvent(LayerEvent::AddLayer(self.layer_to)),
        ))
    }

    fn on_deactivate(&self) -> Option<Event> {
        Some(Event::new(
            self.behavior_id,
            EventData::LayerEvent(LayerEvent::RemoveToLayer(self.layer_from)),
        ))
    }

    fn behavior_id(&self) -> usize {
        self.behavior_id
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

    fn id(&self) -> usize {
        match self {
            ManualBehavior::HoldTap(b) => b.id(),
            ManualBehavior::Simple(b) => b.behavior_id(),
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
    id: usize,
    state: HoldTapBehaviorState,
    hold: SimpleBehavior,
    tap: SimpleBehavior,
    timeout: Duration,
    hold_while_undecided: bool,
}

impl HoldTapBehavior {
    pub fn new(
        hold: SimpleBehavior,
        tap: SimpleBehavior,
        timeout: Duration,
        hold_while_undecided: bool,
    ) -> Self {
        Self {
            id: get_behavior_id(),
            state: HoldTapBehaviorState::Pending,
            tap,
            hold,
            timeout,
            hold_while_undecided,
        }
    }
}

impl BehaviorComplex for HoldTapBehavior {
    fn on_press(&mut self) -> Option<Event> {
        if self.hold_while_undecided {
            Some(Event::new(
                self.id,
                EventData::BehaviorEvent(BehaviorEvent::StartBehavior(self.hold)),
            ))
        } else {
            None
        }
    }

    fn on_unpress(&mut self) -> Option<Event> {
        match self.state {
            HoldTapBehaviorState::DecidedTap => {
                panic!("Invalid state: DecidedTap encountered in on_deactivate")
            }
            HoldTapBehaviorState::DecidedHold => Some(Event::new(
                self.id,
                EventData::BehaviorEvent(BehaviorEvent::EndBehavior(self.hold)),
            )),
            HoldTapBehaviorState::Pending => {
                self.state = HoldTapBehaviorState::DecidedTap;

                if self.hold_while_undecided {
                    Some(Event::new(
                        self.id,
                        EventData::BehaviorEvent(BehaviorEvent::ReleaseTapBehavior(
                            self.hold, self.tap,
                        )),
                    ))
                } else {
                    Some(Event::new(
                        self.id,
                        EventData::BehaviorEvent(BehaviorEvent::TapBehavior(self.tap)),
                    ))
                }
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
                    Some(Event::new(
                        self.id,
                        EventData::BehaviorEvent(BehaviorEvent::StartBehavior(self.hold)),
                    ))
                }
            }
            _ => None,
        }
    }

    fn id(&self) -> usize {
        self.id
    }
}

const BEHAVIOR_ID: Cell<usize> = Cell::new(1);
pub fn get_behavior_id() -> usize {
    let val = BEHAVIOR_ID.get();
    BEHAVIOR_ID.set(val + 1);
    val
}
