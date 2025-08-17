use crate::event::Event;

pub const EVENT_QUEUE_LEN: usize = 100;

pub struct EventQueue {
    events: [Option<Event>; EVENT_QUEUE_LEN],
    cursor: usize,
    len: usize,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: [None; EVENT_QUEUE_LEN],
            cursor: 0,
            len: 0,
        }
    }

    pub fn push_back(&mut self, event: Event) {
        if self.len >= EVENT_QUEUE_LEN {
            panic!("Tried to push to full event queue");
        }

        let index = (self.cursor + self.len) % EVENT_QUEUE_LEN;

        self.events[index] = Some(event);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<Event> {
        let res = self.events[self.cursor].take();

        if self.len > 0 {
            self.len -= 1;
        }

        if self.len == 0 {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }

        if self.cursor >= EVENT_QUEUE_LEN {
            self.cursor = 0;
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use crate::key::Key;

    use super::*;

    #[test]
    fn test_event_queue_basics() {
        let mut eq = EventQueue::new();
        let def_arr = eq.events.clone();

        eq.push_back(Event::key_up(Key::A));
        eq.push_back(Event::key_up(Key::B));

        let mut exp = def_arr.clone();
        exp[0] = Some(Event::key_up(Key::A));
        exp[1] = Some(Event::key_up(Key::B));
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 2);
        assert_eq!(eq.cursor, 0);

        assert_eq!(eq.pop_front(), Some(Event::key_up(Key::A)));

        exp[0] = None;
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 1);
        assert_eq!(eq.cursor, 1);

        assert_eq!(eq.pop_front(), Some(Event::key_up(Key::B)));

        exp[1] = None;
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 0);
        assert_eq!(eq.cursor, 0);

        for i in 0..100 {
            match i % 2 {
                0 => eq.push_back(Event::key_up(Key::A)),
                1 => eq.push_back(Event::key_down(Key::A)),
                _ => panic!(
                    "the rust compiler found the long-sought exception to the even/odd binary"
                ),
            }
        }

        assert_eq!(eq.len, 100);
        assert_eq!(eq.cursor, 0);
        for event in eq.events {
            assert!(event.is_some());
        }

        // To avoid recalculating this
        let mut exp = eq.events.clone();

        assert_eq!(eq.pop_front(), Some(Event::key_up(Key::A)));

        exp[0] = None;
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 99);
        assert_eq!(eq.cursor, 1);

        eq.push_back(Event::key_down(Key::A));

        exp[0] = Some(Event::key_down(Key::A));
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 100);
        assert_eq!(eq.cursor, 1);

        assert_eq!(eq.pop_front(), Some(Event::key_down(Key::A)));

        exp[1] = None;
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 99);
        assert_eq!(eq.cursor, 2);

        eq.push_back(Event::key_up(Key::B));
        exp[1] = Some(Event::key_up(Key::B));
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 100);
        assert_eq!(eq.cursor, 2);

        for _ in 0..99 {
            eq.pop_front();
        }

        let mut exp = def_arr.clone();
        exp[1] = Some(Event::key_up(Key::B));
        assert_eq!(eq.events, exp);
        assert_eq!(eq.len, 1);
        assert_eq!(eq.cursor, 1);
    }
}
