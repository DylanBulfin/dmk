use crate::event::Event;

pub const EVENT_QUEUE_LEN: usize = 100;

pub struct EventQueue<'b> {
    events: [Option<Event<'b>>; EVENT_QUEUE_LEN],
    cursor: usize,
    len: usize,
}

impl<'b> EventQueue<'b> {
    pub fn push_back(&mut self, event: Event<'b>) {
        if self.len >= EVENT_QUEUE_LEN {
            panic!("Tried to push to full event queue");
        }

        let index = (self.cursor + self.len) % EVENT_QUEUE_LEN;

        self.events[index] = Some(event);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<Event<'b>> {
        let res = self.events[self.cursor].take();

        if self.len > 0 {
            self.len -= 1;
        }

        if self.len == 0 {
            self.cursor = 0;
        }

        res
    }
}

