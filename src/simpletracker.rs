
use Tracker;

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
fn it_works() {}
