// Generated macro for impl_3423 (impl)
macro_rules! Depcrate_timeimpl_3423 {
() => {
// Module: crate::time
// Provides: {"impl_3423"}
// Dependencies: {}
impl SystemTimeError { # [doc = " Returns the positive duration which represents how far forward the"] # [doc = " second system time was from the first."] # [doc = ""] # [doc = " A `SystemTimeError` is returned from the [`SystemTime::duration_since`]"] # [doc = " and [`SystemTime::elapsed`] methods whenever the second system time"] # [doc = " represents a point later in time than the `self` of the method call."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::thread::sleep;"] # [doc = " use std::time::{Duration, SystemTime};"] # [doc = ""] # [doc = " let sys_time = SystemTime::now();"] # [doc = " sleep(Duration::from_secs(1));"] # [doc = " let new_sys_time = SystemTime::now();"] # [doc = " match sys_time.duration_since(new_sys_time) {"] # [doc = "     Ok(_) => {}"] # [doc = "     Err(e) => println!(\"SystemTimeError difference: {:?}\", e.duration()),"] # [doc = " }"] # [doc = " ```"] # [must_use] # [stable (feature = "time2" , since = "1.8.0")] pub fn duration (& self) -> Duration { self . 0 } }
};
}
