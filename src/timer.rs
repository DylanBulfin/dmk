use core::cmp::Ordering;

use crate::vboard::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    microseconds: u64,
}

impl Duration {
    pub fn from_micros(micros: u64) -> Self {
        Self {
            microseconds: micros,
        }
    }

    pub fn from_millis(millis: u64) -> Self {
        Self {
            microseconds: millis * 1000,
        }
    }

    pub fn from_secs(secs: u64) -> Self {
        Self {
            microseconds: secs * 1_000_000,
        }
    }

    pub fn micros(&self) -> u64 {
        self.microseconds
    }

    pub fn millis(&self) -> u64 {
        self.microseconds / 1000
    }

    pub fn secs(&self) -> u64 {
        self.microseconds / 1_000_000
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    microseconds: u64,
}

impl Instant {
    pub fn from_micros(micros: u64) -> Self {
        Self {
            microseconds: micros,
        }
    }

    pub fn from_millis(millis: u64) -> Self {
        Self {
            microseconds: millis * 1000,
        }
    }

    pub fn from_secs(secs: u64) -> Self {
        Self {
            microseconds: secs * 1_000_000,
        }
    }

    pub fn micros(&self) -> u64 {
        self.microseconds
    }

    pub fn millis(&self) -> u64 {
        self.microseconds / 1000
    }

    pub fn secs(&self) -> u64 {
        self.microseconds / 1_000_000
    }
}

pub trait Timer {
    fn as_instant(&self) -> Instant;
    fn add_duration(&self, duration: Duration) -> Instant;
    fn wait(&self, duration: Duration);
}

pub enum TimerEvent {
    Behavior(BehaviorTimeoutEvent),
    /// System-created key release event
    UntapKey(UntapKeyEvent),
    UntapBehavior(UntapBehaviorEvent),
}

impl TimerEvent {
    pub fn instant(&self) -> Instant {
        match self {
            TimerEvent::Behavior(e) => e.instant,
            Self::UntapKey(t) => t.instant,
            Self::UntapBehavior(t) => t.instant,
        }
    }
}

impl PartialEq for TimerEvent {
    fn eq(&self, other: &Self) -> bool {
        self.instant() == other.instant()
    }
}

impl Eq for TimerEvent {}

impl PartialOrd for TimerEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs_inst = match self {
            TimerEvent::Behavior(e) => e.instant,
            Self::UntapKey(t) => t.instant,
            Self::UntapBehavior(t) => t.instant,
        };
        let rhs_inst = match other {
            TimerEvent::Behavior(e) => e.instant,
            Self::UntapKey(t) => t.instant,
            Self::UntapBehavior(t) => t.instant,
        };

        lhs_inst.cmp(&rhs_inst)
    }
}

pub struct BehaviorTimeoutEvent {
    pub behavior_id: usize,
    pub instant: Instant,
}

impl PartialEq for BehaviorTimeoutEvent {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

pub struct UntapBehaviorEvent {
    pub behavior_id: usize,
    pub instant: Instant,
}

impl PartialEq for UntapBehaviorEvent {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

pub struct UntapKeyEvent {
    pub key: Key,
    pub instant: Instant,
}

impl PartialEq for UntapKeyEvent {
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}
