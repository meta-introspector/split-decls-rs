// Generated macro for UNIX_EPOCH (const)
macro_rules! Depcrate_timeUNIX_EPOCH {
() => {
// Module: crate::time
// Provides: {"UNIX_EPOCH"}
// Dependencies: {}
# [doc = " An anchor in time which can be used to create new `SystemTime` instances or"] # [doc = " learn about where in time a `SystemTime` lies."] # [doc = ""] # [doc = " This constant is defined to be \"1970-01-01 00:00:00 UTC\" on all systems with"] # [doc = " respect to the system clock. Using `duration_since` on an existing"] # [doc = " [`SystemTime`] instance can tell how far away from this point in time a"] # [doc = " measurement lies, and using `UNIX_EPOCH + duration` can be used to create a"] # [doc = " [`SystemTime`] instance to represent another fixed point in time."] # [doc = ""] # [doc = " `duration_since(UNIX_EPOCH).unwrap().as_secs()` returns"] # [doc = " the number of non-leap seconds since the start of 1970 UTC."] # [doc = " This is a POSIX `time_t` (as a `u64`),"] # [doc = " and is the same time representation as used in many Internet protocols."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::time::{SystemTime, UNIX_EPOCH};"] # [doc = ""] # [doc = " match SystemTime::now().duration_since(UNIX_EPOCH) {"] # [doc = "     Ok(n) => println!(\"1970-01-01 00:00:00 UTC was {} seconds ago!\", n.as_secs()),"] # [doc = "     Err(_) => panic!(\"SystemTime before UNIX EPOCH!\"),"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "time2" , since = "1.8.0")] pub const UNIX_EPOCH : SystemTime = SystemTime (time :: UNIX_EPOCH) ;
};
}
