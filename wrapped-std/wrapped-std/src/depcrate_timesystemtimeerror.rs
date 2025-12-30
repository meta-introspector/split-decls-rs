// Generated macro for SystemTimeError (struct)
macro_rules! Depcrate_timeSystemTimeError {
() => {
// Module: crate::time
// Provides: {"SystemTimeError"}
// Dependencies: {}
# [doc = " An error returned from the `duration_since` and `elapsed` methods on"] # [doc = " `SystemTime`, used to learn how far in the opposite direction a system time"] # [doc = " lies."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::thread::sleep;"] # [doc = " use std::time::{Duration, SystemTime};"] # [doc = ""] # [doc = " let sys_time = SystemTime::now();"] # [doc = " sleep(Duration::from_secs(1));"] # [doc = " let new_sys_time = SystemTime::now();"] # [doc = " match sys_time.duration_since(new_sys_time) {"] # [doc = "     Ok(_) => {}"] # [doc = "     Err(e) => println!(\"SystemTimeError difference: {:?}\", e.duration()),"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug)] # [stable (feature = "time2" , since = "1.8.0")] pub struct SystemTimeError (Duration) ;
};
}
