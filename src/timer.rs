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
