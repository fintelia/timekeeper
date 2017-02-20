
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::ops::Index;

use std::time;

use Timer;

/// A timer set tracks a collection of timers, of which at most one can be
/// running at any given time.
pub struct TimerSet<Key: Eq + Hash + Clone, T: Timer + Default> {
    timers: HashMap<Key, T>,
    current: Option<Key>
}

impl <Key: Eq + Hash + Clone, T: Timer + Default> TimerSet<Key, T> {
    /// Create a new TimerSet with no currently running timer.
    pub fn new() -> Self {
        Self {
            timers: HashMap::new(),
            current: None,
        }
    }

    /// Starts a specific timer, stopping the currently running timer (if any).
    pub fn start(&mut self, k: Key) {
        let now = time::Instant::now();

        if let Some(ref k) = self.current {
            self.timers.get_mut(k).unwrap().stop_at(now);
        }

        self.current = Some(k.clone());
        self.timers.entry(k).or_insert_with(T::default).start_at(now);
    }

    /// Stops the currently running timer without starting any others.
    pub fn stop(&mut self) {
        if let Some(ref k) = self.current {
            self.timers.get_mut(k).unwrap().stop();
        }
        self.current = None;
    }

    /// Get a reference to timer k.
    pub fn get(&self, k: &Key) -> Option<&T> {
        self.timers.get(k)
    }
}

impl<Key: Eq + Hash + Clone, T: Timer + Default> Index<Key> for TimerSet<Key, T> {
    type Output = T;

    fn index(&self, k: Key) -> &T {
        self.timers.get(&k).expect("Attempted to get timer that doesn't exist")
    }
}

#[test]
fn it_works() {
    use simpletimer::SimpleTimer;

    let mut ts = TimerSet::<i32, SimpleTimer>::new();

    ts.start(1);
    ts.start(2);
    ts.stop();

    assert!(ts[1].num_nanoseconds() > 0);
    assert!(ts[2].num_nanoseconds() > 0);
    assert!(ts.get(&3).is_none());
}
