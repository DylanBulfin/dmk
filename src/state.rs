use core::{marker::PhantomData, ops::Index};

use crate::{
    behavior::{Behavior, DefaultBehavior, KeyState},
    event::{Event, LayerEvent, SpecialEvent, queue::EventQueue},
    layer::{Layer, LayerStack},
    physical_layout::{HeldKeyCollection, MAX_KEYS, PhysicalLayout},
    timer::{Duration, Timer, TimerQueue, TimerTrigger, TimerTriggerData},
    vboard::VirtualKeyboard,
};

pub struct State<P, C, T>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer>,
    T: Timer,
{
    layer_stack: LayerStack<C>,
    phys_state: PhysicalState<P>,
    virtual_board: VirtualKeyboard,
    event_queue: EventQueue,
    key_state: KeyState,
    pub timer_state: TimerState<T>,
}

struct PhysicalState<P>
where
    P: PhysicalLayout,
{
    layout: P,
    last_state: [bool; MAX_KEYS],
    held_keys: HeldKeyCollection,
}

pub struct TimerState<T>
where
    T: Timer,
{
    pub timer: T,
    queue: TimerQueue,
}

/// Time to hold a key when the state manager is simulating a tap
pub const TAP_DURATION_MS: u64 = 100;

impl<P, C, T> State<P, C, T>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer>,
    T: Timer,
{
    pub fn new(layers: C, layout: P, timer: T) -> Self {
        let layer_stack = LayerStack::new(layers);
        let phys_state = PhysicalState {
            layout,
            last_state: [false; MAX_KEYS],
            held_keys: HeldKeyCollection::new(),
        };
        let virtual_board = VirtualKeyboard::default();
        let event_queue = EventQueue::new();
        let key_state = KeyState {};
        let timer_state = TimerState {
            timer,
            queue: TimerQueue::new(),
        };

        Self {
            layer_stack,
            phys_state,
            virtual_board,
            event_queue,
            key_state,
            timer_state,
        }
    }

    pub fn main_iteration(&mut self) {
        self.handle_timer_events();
        // TODO consider whether to handle one event per loop instead
        while let Some(event) = self.event_queue.pop_front() {
            self.handle_event(event);
        }
        self.handle_physical_key_state();
    }

    pub fn main_loop(&mut self) {
        loop {
            self.main_iteration();
        }
    }

    /// Execute a single event depending on the type
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::BehaviorKeyEvent(mut bke) => {
                let events = if bke.is_press {
                    bke.behavior.on_press(&self.key_state)
                } else {
                    bke.behavior.on_release(&self.key_state)
                };

                self.event_queue.push_evec(events);

                if bke.is_press
                    && let Some(duration) = bke.behavior.try_get_delay()
                {
                    self.timer_state.queue.insert(TimerTrigger {
                        time: self.timer_state.timer.add_duration(duration),
                        data: TimerTriggerData::Behavior(bke.behavior),
                    });
                }
            }
            Event::KeyEvent(ke) => {
                self.virtual_board.apply_key_event(ke);
            }
            Event::LayerEvent(le) => match le {
                LayerEvent::AddLayer(layer) => self.layer_stack.push(layer),
                LayerEvent::RemoveDownToLayer(layer) => self.layer_stack.pop_until(layer),
            },
            Event::SpecialEvent(se) => match se {
                SpecialEvent::TapBehavior(mut behavior) => {
                    let events = behavior.on_press(&self.key_state);
                    self.event_queue.push_evec(events);

                    self.timer_state.queue.insert(TimerTrigger::event(
                        self.timer_state
                            .timer
                            .add_duration(Duration::from_millis(TAP_DURATION_MS)),
                        Event::bkey_up(behavior),
                    ));
                }
            },
            Event::None => {}
        }
    }

    /// Check for any timer events in the timer queue that should be triggered at this Instant, and
    /// trigger their after_delay methods
    pub fn handle_timer_events(&mut self) {
        while let Some(event) = self.timer_state.queue.peek_front()
            && event.time <= self.timer_state.timer.as_instant()
            && let Some(event) = self.timer_state.queue.pop_front()
        {
            match event.data {
                TimerTriggerData::Behavior(mut behavior) => {
                    let mut target_key: Option<usize> = None;
                    for (key, beh) in self.phys_state.held_keys.iter() {
                        if beh == &behavior {
                            target_key = Some(*key);
                        }
                    }

                    let events = behavior.after_delay(&self.key_state);
                    self.event_queue.push_evec(events);

                    if let Some(key) = target_key {
                        self.phys_state.held_keys.replace(key, behavior);
                    }
                }
                TimerTriggerData::Event(event) => self.event_queue.push_back(event),
            }
        }
    }

    /// Check the state of each physical key in the configured layout, and generate
    /// BehaviorKeyUp/BehaviorKeyDown behaviors for newly released/pressed keys
    pub fn handle_physical_key_state(&mut self) {
        let curr_state = self.phys_state.layout.get_keys(&self.timer_state.timer);

        for key in 0..self.phys_state.layout.keys() {
            if curr_state[key] && !self.phys_state.last_state[key] {
                // Key newly pressed
                // TODO add debouncing, maybe new event type
                let behavior = self.layer_stack.find_key_behavior(key);
                self.event_queue.push_back(Event::bkey_down(behavior));
                self.phys_state.held_keys.push(key, behavior);
            } else if !curr_state[key] && self.phys_state.last_state[key] {
                // Newly released key
                let behavior = if let Some(beh) = self.phys_state.held_keys.try_remove_key(key) {
                    beh
                } else {
                    panic!("Got release event of non-held key");
                };
                self.event_queue.push_back(Event::bkey_up(behavior));
            }
        }

        self.phys_state.last_state = curr_state;
    }

    pub fn get_vboard(&self) -> &VirtualKeyboard {
        &self.virtual_board
    }

    pub fn get_phys_layout_mut(&mut self) -> &mut P {
        &mut self.phys_state.layout
    }
}
