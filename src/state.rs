use core::ops::Index;

use crate::{
    behavior::{Behavior, DefaultBehavior, KeyState},
    event::{Event, queue::EventQueue},
    layer::{Layer, LayerStack},
    physical_layout::PhysicalLayout,
    timer::Duration,
    virtual_board::VirtualKeyboard,
};

pub struct TimerQueue {}

impl TimerQueue {
    pub fn add(&mut self, duration: Duration, behavior: DefaultBehavior) {
        unimplemented!()
    }
}

pub struct State<P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    layers: C,
    layout: P,
    virtual_board: VirtualKeyboard,
    layer_stack: LayerStack<P>,
    event_queue: EventQueue,
    key_state: KeyState,
    timer_queue: TimerQueue,
}

impl<P, C> State<P, C>
where
    P: PhysicalLayout,
    C: Index<usize, Output = Layer<P>>,
{
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::BehaviorKeyEvent(bke) => {
                let mut behavior: DefaultBehavior = bke.behavior.clone();
                let events = if bke.is_press {
                    behavior.on_press(&self.key_state)
                } else {
                    behavior.on_release(&self.key_state)
                };

                for event in events {
                    self.event_queue.push_back(event);
                }

                if let Some(dur) = behavior.try_get_delay() {
                    self.timer_queue.add(dur, behavior);
                }
            }
            Event::KeyEvent(ke) => {
                self.virtual_board.apply_key_event(ke);
            }
            Event::LayerEvent(le) => todo!(),
            Event::SpecialEvent(se) => todo!(),
            Event::None => todo!(),
        }
    }
}
