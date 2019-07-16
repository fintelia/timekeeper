
use crate::Tracker;

pub struct SimpleTracker {
    elapsed: u64,
}

impl Default for SimpleTracker {
    fn default() -> Self {
        Self {
            elapsed: 0,
        }
    }
}

impl Tracker for SimpleTracker {
    type Statistics = u64;

    fn record(&mut self, time: u64) {
        self.elapsed += time;
    }

    fn get_stats(&self, partial_time: Option<u64>) -> Self::Statistics {
        self.elapsed + partial_time.unwrap_or(0)
    }

    fn get(&self, partial_time: Option<u64>) -> u64 {
        self.get_stats(partial_time)
    }
}

#[test]
fn it_works() {
    let mut tracker = SimpleTracker::default();

    tracker.record(10);
    tracker.record(12);
    tracker.record(0);

    assert_eq!(tracker.get(Some(5)), 27);
    assert_eq!(tracker.get(None), 22);

    tracker.record(18);

    assert_eq!(tracker.get(None), 40);
    assert_eq!(tracker.get_stats(None), 40);
}
