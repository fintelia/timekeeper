
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
fn getrusage(resource: libc::c_int) -> Result<libc::rusage, ()> {
    let mut usage: libc::rusage = libc::rusage {
        ru_utime: libc::timeval{tv_sec: 0, tv_usec: 0},
        ru_stime: libc::timeval{tv_sec: 0, tv_usec: 0},
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };
    let ret = unsafe {
        libc::getrusage(resource, &mut usage)
    };

    if ret == 0 {
        Ok(usage)
    } else {
        Err(())
    }
}

#[derive(Default)]
pub struct ProcessTime;
impl Source for ProcessTime {
    fn get_time(&self) -> u64 {
        let usage = getrusage(libc::RUSAGE_SELF).unwrap();
        let secs = usage.ru_utime.tv_sec + usage.ru_stime.tv_sec;
        let usecs = usage.ru_utime.tv_usec + usage.ru_stime.tv_usec;
        (secs as u64) * 1000_000_000 + (usecs as u64) * 1000
    }
}

#[derive(Default)]
pub struct ThreadTime;
impl Source for ThreadTime {
    fn get_time(&self) -> u64 {
        let usage = getrusage(libc::RUSAGE_THREAD).unwrap();
        let secs = usage.ru_utime.tv_sec + usage.ru_stime.tv_sec;
        let usecs = usage.ru_utime.tv_usec + usage.ru_stime.tv_usec;
        (secs as u64) * 1000_000_000 + (usecs as u64) * 1000
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

    // ProcessTime may have lower precision (on the order of microseconds) so we
    // have to do some work to get the difference to be noticable.
    let source = ProcessTime::default();
    let t1 = source.get_time();
    do_work(1000000);
    let t2 = source.get_time();
    assert!(t2 > t1);

    // Same test for ThreadTime.
    let source = ThreadTime::default();
    let t1 = source.get_time();
    do_work(1000000);
    let t2 = source.get_time();
    assert!(t2 > t1);
}
