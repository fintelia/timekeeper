//! Timekeeper is a simple library to track the amount of time used by different
//! parts of a program.

extern crate libc;

mod simpletracker;
mod source;

pub use simpletracker::SimpleTracker;
pub use source::*;

#[cfg(feature = "enable_timekeeper")] mod timer;
#[cfg(feature = "enable_timekeeper")] mod timerset;
#[cfg(feature = "enable_timekeeper")] pub use timer::*;
#[cfg(feature = "enable_timekeeper")] pub use timerset::*;

#[cfg(not(feature = "enable_timekeeper"))] mod nop_timer;
#[cfg(not(feature = "enable_timekeeper"))] mod nop_timerset;
#[cfg(not(feature = "enable_timekeeper"))] pub use nop_timer::*;
#[cfg(not(feature = "enable_timekeeper"))] pub use nop_timerset::*;

pub trait Tracker: Default {
    type Statistics: Default;

    fn record(&mut self, time: u64);
    fn get_stats(&self, partial_time: Option<u64>) -> Self::Statistics;
    fn get(&self, partial_time: Option<u64>) -> u64;
}

pub trait Source: Default {
    fn get_time(&self) -> u64;
}
