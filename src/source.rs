
use std::time;

use Source;

pub struct RealTime {
    start: time::Instant,
}

impl Default for RealTime {
    fn default() -> RealTime {
        Self {
            start: time::Instant::now()
        }
    }
}

impl Source for RealTime {
    fn get_time(&self) -> u64 {
        let dur = self.start.elapsed();
        dur.as_secs() * 1000_000_000 + (dur.subsec_nanos() as u64)
    }
}
