
use std::cmp::Eq;
use std::hash::Hash;
use std::marker::PhantomData;

use Tracker;
use Source;

pub struct TimerSet<Key: Eq + Hash + Clone, T: Tracker, S: Source> {
    phantom_key: PhantomData<Key>,
    phantom_t: PhantomData<T>,
    phantom_s: PhantomData<S>,
}

impl <Key, T, S> TimerSet<Key, T, S> where Key: Eq + Hash + Clone, T: Tracker, S: Source {
    pub fn new() -> Self {
        Self {
            phantom_key: PhantomData,
            phantom_t: PhantomData,
            phantom_s: PhantomData,
        }
    }

    pub fn start(&mut self, _: Key) {}
    pub fn stop(&mut self) {}
    pub fn is_running(&self) -> bool {
        false
    }
    pub fn get_stats(&self, _: Key) -> Option<T::Statistics> {
        Some(T::Statistics::default())
    }
    pub fn num_nanoseconds(&self, _: Key) -> Option<u64> {Some(0)}
    pub fn num_microseconds(&self, _: Key) -> Option<u64> {Some(0)}
    pub fn num_milliseconds(&self, _: Key) -> Option<u64> {Some(0)}
    pub fn num_seconds(&self, _: Key) -> Option<u64> {Some(0)}
    pub fn num_minutes(&self, _: Key) -> Option<u64> {Some(0)}
    pub fn num_hours(&self, _: Key) -> Option<u64> {Some(0)}
}
