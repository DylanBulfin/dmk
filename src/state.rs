use core::ops::Index;

use crate::{
    behavior::{Behavior, KeyState},
    event::{Event, LayerEvent, SpecialEvent, queue::EventQueue},
    layer::{Layer, LayerStack},
    physical_layout::{MAX_KEYS, PhysicalLayout},
    timer::{Duration, Timer, TimerQueue, TimerTrigger, TimerTriggerData},
    virtual_board::VirtualKeyboard,
};

pub struct State<P, C, T>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
    T: Timer,
{
    layer_state: LayerStack<P, C>,
    phys_state: PhysicalState<P>,
    virtual_board: VirtualKeyboard,
    event_queue: EventQueue,
    key_state: KeyState,
    timer_state: TimerState<T>,
}

pub struct PhysicalState<P>
where
    P: PhysicalLayout,
{
    layout: P,
    last_state: [bool; MAX_KEYS],
}

pub struct TimerState<T>
where
    T: Timer,
{
    timer: T,
    queue: TimerQueue,
}

pub const TAP_DURATION_MS: u64 = 100;

impl<P, C, T> State<P, C, T>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
    T: Timer,
{
    pub fn main_loop(&mut self) {
        loop {
            self.handle_timer_events();
            // TODO consider whether to handle one event per loop instead
            while let Some(event) = self.event_queue.pop_front() {
                self.handle_event(event);
            }
            self.handle_physical_key_state();
        }
    }

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
                LayerEvent::AddLayer(layer) => self.layer_state.push(layer),
                LayerEvent::RemoveDownToLayer(layer) => self.layer_state.pop_until(layer),
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

    pub fn handle_timer_events(&mut self) {
        while let Some(event) = self.timer_state.queue.peek_front()
            && event.time <= self.timer_state.timer.as_instant()
            && let Some(event) = self.timer_state.queue.pop_front()
        {
            match event.data {
                TimerTriggerData::Behavior(mut behavior) => {
                    let events = behavior.after_delay(&self.key_state);
                    self.event_queue.push_evec(events);
                }
                TimerTriggerData::Event(event) => self.event_queue.push_back(event),
            }
        }
    }

    pub fn handle_physical_key_state(&mut self) {
        let curr_state = self.phys_state.layout.get_arr_copy();

        for key in 0..self.phys_state.layout.keys() {
            if curr_state[key] && !self.phys_state.last_state[key] {
                // Key newly pressed
                // TODO add debouncing, maybe new event type
                let behavior = self.layer_state.find_key_behavior(key);
                self.event_queue.push_back(Event::bkey_down(behavior));
            } else if !curr_state[key] && self.phys_state.last_state[key] {
                // Newly released key
                let behavior = self.layer_state.find_key_behavior(key);
                self.event_queue.push_back(Event::bkey_up(behavior));
            }
        }

        self.phys_state.last_state = curr_state;
    }
}
