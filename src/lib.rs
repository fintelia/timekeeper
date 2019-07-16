//! Timekeeper is a simple library to track the amount of time used by different
//! parts of a program.

extern crate libc;

mod simpletracker;
mod source;

pub use crate::simpletracker::SimpleTracker;
pub use crate::source::*;

#[cfg(feature = "enable_timekeeper")] mod timer;
#[cfg(feature = "enable_timekeeper")] mod timerset;
#[cfg(feature = "enable_timekeeper")] pub use crate::timer::*;
#[cfg(feature = "enable_timekeeper")] pub use crate::timerset::*;

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
