use static_collections::{HashMap, PriorityQueue, Queue};

use crate::{
    behavior::{BehaviorComplex, BehaviorSimple, ManualBehavior},
    event::{BehaviorEvent, ComplexKeyEvent, Event, EventData, KeyEvent, SimpleKeyEvent},
    timer::{BehaviorTimeoutEvent, Duration, Timer, TimerEvent, UntapBehaviorEvent, UntapKeyEvent},
    vboard::KeyboardState,
};

pub const MAX_HELD_BEHAVIORS: usize = 20;
const HELD_BEH_BACK_ARR_LEN: usize = MAX_HELD_BEHAVIORS * 2;
pub const MAX_EVENTS: usize = 100;
pub const MAX_TIMER_EVENTS: usize = 50;

pub struct State<T>
where
    T: Timer,
{
    // The number of elements in the HashMap's backing array
    held_behaviors: HashMap<usize, ManualBehavior, HELD_BEH_BACK_ARR_LEN>,
    event_queue: Queue<Event, MAX_EVENTS>,
    timer: T,
    timer_events: PriorityQueue<TimerEvent, MAX_TIMER_EVENTS>,
    keyboard_state: KeyboardState,
}

impl<T> State<T>
where
    T: Timer,
{
    pub fn apply_event(&mut self, event: Event) {
        match event.data {
            EventData::KeyEvent(ke) => {
                match ke {
                    KeyEvent::Complex(e) => match e {
                        ComplexKeyEvent::ReleaseTap(key, key1) => {
                            // Unrelease first key
                            self.keyboard_state.held_keys.remove_by(|(k, _)| *k == key);
                            // Add push event for tapped key
                            self.event_queue.push_back(Event::new(
                                event.behavior_id,
                                EventData::KeyEvent(KeyEvent::Simple(SimpleKeyEvent::Press(key1))),
                            ));
                            // Add timed release event for tapped key
                            self.timer_events
                                .insert(TimerEvent::UntapKey(UntapKeyEvent {
                                    instant: self.timer.add_duration(Duration::from_millis(100)),
                                    key: key1,
                                }));
                        }
                        ComplexKeyEvent::ReleasePress(key, key1) => {
                            // Unrelease first key
                            self.keyboard_state.held_keys.remove_by(|(k, _)| *k == key);
                            // Add push event for tapped key
                            self.event_queue.push_back(Event::new(
                                event.behavior_id,
                                EventData::KeyEvent(KeyEvent::Simple(SimpleKeyEvent::Press(key1))),
                            ));
                        }
                    },
                    KeyEvent::Simple(e) => match e {
                        SimpleKeyEvent::Press(key) => self
                            .keyboard_state
                            .held_keys
                            .push_back((key, event.behavior_id)),
                        SimpleKeyEvent::Unpress(key) => {
                            self.keyboard_state.held_keys.remove_by(|(k, _)| *k == key);
                        }
                    },
                }
            }
            EventData::BehaviorEvent(be) => match be {
                BehaviorEvent::StartBehavior(sb) => {
                    if let Some(event) = sb.on_activate() {
                        self.event_queue.push_back(event);
                    }

                    if let Some(dur) = sb.get_duration() {
                        self.timer_events
                            .insert(TimerEvent::Behavior(BehaviorTimeoutEvent {
                                behavior_id: event.behavior_id,
                                instant: self.timer.add_duration(dur),
                            }));
                    }
                }
                BehaviorEvent::EndBehavior(sb) => {
                    if let Some(event) = sb.on_deactivate() {
                        self.event_queue.push_back(event);
                    }

                    // Remove from held behaviors list,
                    self.held_behaviors.remove(&event.behavior_id);
                }
                BehaviorEvent::TapBehavior(sb) => {
                    self.event_queue.push_back(Event::new(
                        event.behavior_id,
                        EventData::BehaviorEvent(BehaviorEvent::StartBehavior(sb)),
                    ));

                    self.timer_events
                        .insert(TimerEvent::UntapBehavior(UntapBehaviorEvent {
                            behavior_id: event.behavior_id,
                            instant: self.timer.add_duration(Duration::from_millis(100)),
                        }));
                }
                BehaviorEvent::ReleasePressBehavior(sb, sb2) => {
                    self.event_queue.push_back(Event::new(
                        event.behavior_id,
                        EventData::BehaviorEvent(BehaviorEvent::EndBehavior(sb)),
                    ));

                    self.event_queue.push_back(Event::new(
                        event.behavior_id,
                        EventData::BehaviorEvent(BehaviorEvent::EndBehavior(sb2)),
                    ));
                }
                BehaviorEvent::ReleaseTapBehavior(sb, sb2) => {
                    self.event_queue.push_back(Event::new(
                        event.behavior_id,
                        EventData::BehaviorEvent(BehaviorEvent::EndBehavior(sb)),
                    ));

                    self.event_queue.push_back(Event::new(
                        event.behavior_id,
                        EventData::BehaviorEvent(BehaviorEvent::TapBehavior(sb2)),
                    ));
                }
            },
            EventData::LayerEvent(layer_event) => todo!(),
        }
    }

    pub fn apply_timer_event(&mut self, event: TimerEvent) {
        match event {
            TimerEvent::Behavior(e) => {
                if let Some(b) = self.held_behaviors.get_mut(&e.behavior_id) {
                    if let Some(event) = b.on_timeout() {
                        self.event_queue.push_back(event);
                    }
                }
            }
            TimerEvent::UntapKey(e) => {
                self.keyboard_state
                    .held_keys
                    .remove_by(|(k, id)| *k == e.key && *id == 0);
            }
            TimerEvent::UntapBehavior(e) => {
                self.event_queue.push_back(Event::new(e.behavior_id, EventData::BehaviorEvent(BehaviorEvent::EndBehavior()));
            }
        }
    }
}
