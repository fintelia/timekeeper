
use std::time;
use Timer;

pub struct SimpleTimer {
    start: Option<time::Instant>,
    elapsed: time::Duration,
}

impl Default for SimpleTimer {
    fn default() -> Self {
        Self {
            start: None,
            elapsed: time::Duration::from_secs(0),
        }
    }
}

impl Timer for SimpleTimer {
    fn start_at(&mut self, instant: time::Instant) {
        assert!(self.start.is_none(), "Cannot start running timer");
        self.start = Some(instant);
    }

    fn stop_at(&mut self, instant: time::Instant) {
        assert!(self.start.is_some(), "Cannot stop paused timer");
        self.elapsed += instant.duration_since(self.start.unwrap());
        self.start = None;
    }

    fn get(&self) -> time::Duration {
        match self.start {
            Some(i) => self.elapsed + i.elapsed(),
            None => self.elapsed,
        }
    }
}

#[test]
fn it_works() {
    use std::thread::sleep;

    let mut t = SimpleTimer::default();
    assert_eq!(t.num_nanoseconds(), 0);

    t.start();
    t.stop();
    assert!(t.num_nanoseconds() > 0);
    assert_eq!(t.num_milliseconds(), 0);

    t.start();
    sleep(time::Duration::from_millis(100));
    t.stop();
    assert!(t.num_milliseconds() >= 100);
    assert!(t.num_microseconds() <= 102000);
}
