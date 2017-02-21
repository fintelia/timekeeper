//! Timekeeper is a simple library to track the amount of time used by different
//! parts of a program.

#![feature(associated_type_defaults)]

mod simpletracker;
mod source;
mod timerset;

pub use simpletracker::SimpleTracker;
pub use source::*;
pub use timerset::*;

pub trait Tracker: Default {
    type Statistics;

    fn record(&mut self, time: u64);
    fn get_stats(&self, partial_time: Option<u64>) -> Self::Statistics;
    fn get(&self, partial_time: Option<u64>) -> u64;
}

pub trait Source: Default {
    fn get_time(&self) -> u64;
}

pub struct Timer<T: Tracker, S: Source> {
    tracker: T,
    source: S,
    last: Option<u64>,
}

impl<T: Tracker, S: Source> Timer<T, S> {
    /// Create a new Timer.
    pub fn new() -> Self {
        Self {
            tracker: T::default(),
            source: S::default(),
            last: None,
        }
    }

    /// Start timing. Panics if the timer is already running.
    pub fn start(&mut self) {
        assert!(self.last.is_none(), "cannot start running timer");
        self.last = Some(self.source.get_time());
    }

    /// Stop timing. Panics if the timer is not currently running.
    pub fn stop(&mut self) {
        let time = self.source.get_time() - self.last.take().expect("cannot stop paused timer");
        self.tracker.record(time);
    }

    pub fn get_stats(&self) -> T::Statistics {
        self.tracker.get_stats(self.last.map(|l| self.source.get_time() - l))
    }

    /// Returns the elapsed time in nanoseconds.
    pub fn num_nanoseconds(&self) -> u64 {
        self.tracker.get(self.last.map(|l| self.source.get_time() - l))
    }

    /// Returns the elapsed time in microseconds.
    pub fn num_microseconds(&self) -> u64 {
        self.num_nanoseconds() / 1000
    }

    /// Return the elapsed time in milliseconds.
    pub fn num_milliseconds(&self) -> u64 {
        self.num_nanoseconds() / 1000_000
    }

    /// Returns the elapsed time in seconds.
    pub fn num_seconds(&self) -> u64 {
        self.num_nanoseconds() / 1000_000_000
    }

    /// Returns the elapsed time in minutes.
    pub fn num_minutes(&self) -> u64 {
        self.num_nanoseconds() / 60_000_000_000
    }

    /// Returns the elapsed time in hours.
    pub fn num_hours(&self) -> u64 {
        self.num_nanoseconds() / 3600_000_000_000
    }
}
