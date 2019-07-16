
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

use crate::Tracker;
use crate::Source;

/// A timer set tracks a collection of timers, of which at most one can be
/// running at any given time.
#[cfg(not(feature = "disable_timekeeper"))]
pub struct TimerSet<Key: Eq + Hash + Clone, T: Tracker, S: Source> {
    trackers: HashMap<Key, T>,
    source: S,

    current: Option<(Key, u64)>,
}

#[cfg(not(feature = "disable_timekeeper"))]
impl <Key, T, S> TimerSet<Key, T, S> where Key: Eq + Hash + Clone, T: Tracker, S: Source {
    /// Create a new TimerSet with no currently running timer.
    pub fn new() -> Self {
        Self {
            trackers: HashMap::new(),
            source: S::default(),
            current: None,
        }
    }

    /// Starts a specific timer, stopping the currently running timer (if any).
    pub fn start(&mut self, k: Key) {
        let now = self.source.get_time();

        if let Some((ref k, t)) = self.current {
            self.trackers.get_mut(&k).unwrap().record(now - t);
        }

        self.current = Some((k.clone(), now));
        self.trackers.entry(k).or_insert_with(T::default);
    }

    /// Stops the currently running timer without starting any others.
    pub fn stop(&mut self) {
        if let Some((ref k, t)) = self.current {
            self.trackers.get_mut(&k).unwrap().record(self.source.get_time() - t);
        }
        self.current = None;
    }

    pub fn get_stats(&self, k: Key) -> Option<T::Statistics> {
        let now = match self.current {
            Some((ref current, t)) if *current == k => Some(self.source.get_time() - t),
            _ => None,
        };

        self.trackers.get(&k).map(|t| t.get_stats(now))
    }

    /// Returns the elapsed time in nanoseconds.
    pub fn num_nanoseconds(&self, k: Key) -> Option<u64> {
        let now = match self.current {
            Some((ref current, t)) if *current == k => Some(self.source.get_time() - t),
            _ => None,
        };

        self.trackers.get(&k).map(|t| t.get(now))
    }

    /// Returns the elapsed time in microseconds.
    pub fn num_microseconds(&self, k: Key) -> Option<u64> {
        self.num_nanoseconds(k).map(|t| t / 1000)
    }

    /// Return the elapsed time in milliseconds.
    pub fn num_milliseconds(&self, k: Key) -> Option<u64> {
        self.num_nanoseconds(k).map(|t| t / 1000_000)
    }

    /// Returns the elapsed time in seconds.
    pub fn num_seconds(&self, k: Key) -> Option<u64> {
        self.num_nanoseconds(k).map(|t| t / 1000_000_000)
    }

    /// Returns the elapsed time in minutes.
    pub fn num_minutes(&self, k: Key) -> Option<u64> {
        self.num_nanoseconds(k).map(|t| t / 60_000_000_000)
    }

    /// Returns the elapsed time in hours.
    pub fn num_hours(&self, k: Key) -> Option<u64> {
        self.num_nanoseconds(k).map(|t| t / 3600_000_000_000)
    }
}

#[test]
fn it_works() {
    use crate::SimpleTracker;
    use crate::RealTime;

    let mut ts = TimerSet::<i32, SimpleTracker, RealTime>::new();

    ts.start(1);
    ts.start(2);
    ts.stop();

    assert!(ts.num_nanoseconds(1).unwrap() > 0);
    assert!(ts.num_nanoseconds(2).unwrap() > 0);
    assert!(ts.num_nanoseconds(3).is_none());
}
