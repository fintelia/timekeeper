
use std::marker::PhantomData;

use Source;
use Tracker;

pub struct Timer<T: Tracker, S: Source> {
    phantom_t: PhantomData<T>,
    phantom_s: PhantomData<S>,
}

impl<T: Tracker, S: Source> Timer<T, S> {
    /// Create a new Timer.
    pub fn new() -> Self {
        Self {
            phantom_t: PhantomData,
            phantom_s: PhantomData,
        }
    }

    pub fn start(&mut self) {}
    pub fn stop(&mut self) {}
    pub fn get_stats(&self) -> T::Statistics {
        T::Statistics::default()
    }
    pub fn num_nanoseconds(&self) -> u64 {0}
    pub fn num_microseconds(&self) -> u64 {0}
    pub fn num_milliseconds(&self) -> u64 {0}
    pub fn num_seconds(&self) -> u64 {0}
    pub fn num_minutes(&self) -> u64 {0}
    pub fn num_hours(&self) -> u64 {0}
}

