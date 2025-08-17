//! Should be a purely implementation-independent timing implementation. The obvious way to handle
//! this is to define the system time as a number of nanoseconds since boot. Not all controllers
//! will have that level of precision but it should provide flexibility

use core::{mem, ops::Add};

use crate::{
    behavior::{self, DefaultBehavior},
    event::Event,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    microseconds: u32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Duration {
    microseconds: u32,
}

impl Duration {
    pub fn new(microseconds: u32) -> Self {
        Self { microseconds }
    }

    pub fn from_millis(millis: u32) -> Self {
        Self {
            microseconds: millis * 1000,
        }
    }

    pub fn millis(&self) -> u32 {
        self.microseconds / 1000
    }

    pub fn micros(&self) -> u32 {
        self.microseconds
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self::Output {
            microseconds: self.microseconds + rhs.microseconds,
        }
    }
}

pub trait Timer {
    fn microseconds(&self) -> u32;

    fn as_instant(&self) -> Instant {
        Instant {
            microseconds: self.microseconds(),
        }
    }

    fn add_duration(&self, duration: Duration) -> Instant {
        self.as_instant() + duration
    }
}

pub const TIMER_QUEUE_LEN: usize = 100;

pub struct TimerTrigger {
    pub time: Instant,
    pub data: TimerTriggerData,
}

pub enum TimerTriggerData {
    Behavior(DefaultBehavior),
    Event(Event),
}

impl TimerTrigger {
    pub fn behavior(time: Instant, behavior: DefaultBehavior) -> Self {
        Self {
            time,
            data: TimerTriggerData::Behavior(behavior),
        }
    }

    pub fn event(time: Instant, event: Event) -> Self {
        Self {
            time,
            data: TimerTriggerData::Event(event),
        }
    }
}

/// This structure should always be sorted by TimerEvent::time. so that popping an element from the
/// front will always return it in the correct place.
pub struct TimerQueue {
    arr: [Option<TimerTrigger>; TIMER_QUEUE_LEN],
    len: usize,
}

impl TimerQueue {
    pub fn insert(&mut self, elem: TimerTrigger) {
        if self.len == TIMER_QUEUE_LEN {
            panic!("Attempt to add to a full timer queue");
        }

        // TODO rewrite this to use binary search to find place for element
        let mut i = 0;
        let spot = loop {
            if i >= self.len {
                break self.len;
            }

            if let Some(te) = &self.arr[i] {
                if te.time > elem.time {
                    break 1;
                }
            } else {
                panic!("Unexpected None in TimerQueue")
            }

            i += 1;
        };

        for i in (spot + 1..=self.len).rev() {
            self.arr.swap(i, i - 1);
        }

        self.arr[spot] = Some(elem);
    }

    pub fn pop_front(&mut self) -> Option<TimerTrigger> {
        if self.len == 0 {
            None
        } else {
            let ret = self.arr[0].take().unwrap_or_else(|| {
                panic!(
                    "Unexpected None at head of TimerQueue when len: {}",
                    self.len,
                )
            });

            for i in 1..self.len {
                self.arr.swap(i, i - 1);
            }

            self.len -= 1;

            self.arr[self.len] = None;

            Some(ret)
        }
    }

    pub fn peek_front(&self) -> Option<&TimerTrigger> {
        if self.len == 0 {
            None
        } else {
            Some(self.arr[0].as_ref().unwrap_or_else(|| {
                panic!(
                    "Unexpected None at head of TimerQueue when len: {}",
                    self.len
                )
            }))
        }
    }
}
