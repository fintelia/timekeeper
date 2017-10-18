
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

use libc;
fn clock_gettime(clock: libc::clockid_t) -> Result<libc::timespec, ()> {
    let mut tp: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let ret = unsafe {
        libc::clock_gettime(clock, &mut tp)
    };

    if ret == 0 {
        Ok(tp)
    } else {
        Err(())
    }
}

#[derive(Default)]
pub struct ProcessTime;
impl Source for ProcessTime {
    fn get_time(&self) -> u64 {
        let time = clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID).unwrap();
        (time.tv_sec as u64) * 1000_000_000 + (time.tv_nsec as u64)
    }
}

#[derive(Default)]
pub struct ThreadTime;
impl Source for ThreadTime {
    fn get_time(&self) -> u64 {
        let time = clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID).unwrap();
        (time.tv_sec as u64) * 1000_000_000 + (time.tv_nsec as u64)
    }
}

#[test]
fn it_works() {
    // Sleeping won't work when we're measuring CPU time...
    let do_work = |n|{
        let sum: u64 = (1..n).into_iter().sum();
        assert!(sum > 0);
    };

    // RealTime should have enough precision to notice even the difference
    // between successive readings with nothing in between.
    let source = RealTime::default();
    let t1 = source.get_time();
    let t2 = source.get_time();
    assert!(t2 > t1);

    let realtime = RealTime::default();

    // ProcessTime may have lower precision (on the order of microseconds) so we
    // have to do some work to get the difference to be noticable.
    let source = ProcessTime::default();
    let r1 = realtime.get_time();
    let t1 = source.get_time();
    do_work(1000000);
    let t2 = source.get_time();
    let r2 = realtime.get_time();
    assert!(t2 > t1);
    let load = (t2 - t1) as f64 / (r2 - r1) as f64;
    assert!(load > 0.9 && load < 1.1);

    // Same test for ThreadTime.
    let source = ThreadTime::default();
    let r1 = realtime.get_time();
    let t1 = source.get_time();
    do_work(10000000);
    let t2 = source.get_time();
    let r2 = realtime.get_time();
    assert!(t2 > t1);
    let load = (t2 - t1) as f64 / (r2 - r1) as f64;
    assert!(load > 0.9 && load < 1.1);
}
