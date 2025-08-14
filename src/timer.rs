//! Should be a purely implementation-independent timing implementation. The obvious way to handle
//! this is to define the system time as a number of nanoseconds since boot. Not all controllers
//! will have that level of precision but it should provide flexibility

use core::ops::Add;

pub struct Instant {
    nanoseconds: u64,
}

pub struct Duration {
    nanoseconds: u64,
}

impl Duration {
    pub fn from_micros(micros: u64) -> Self {
        Self {
            nanoseconds: micros * 1000,
        }
    }

    pub fn micros(&self) -> u64 {
        self.nanoseconds / 1000
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self::Output {
            nanoseconds: self.nanoseconds + rhs.nanoseconds,
        }
    }
}

pub trait Timer {
    fn nanoseconds(&self) -> u64;

    fn microseconds(&self) -> u64 {
        self.nanoseconds() / 1000
    }

    fn microseconds_f(&self) -> f64 {
        self.nanoseconds() as f64 / 1000.0
    }

    fn as_instant(&self) -> Instant {
        Instant {
            nanoseconds: self.nanoseconds(),
        }
    }

    fn add_duration(&self, duration: Duration) -> Instant {
        self.as_instant() + duration
    }
}
