// Generated macro for NanosleepRelativeResult (enum)
macro_rules! Depcrate_thread_clockNanosleepRelativeResult {
() => {
// Module: crate::thread::clock
// Provides: {"NanosleepRelativeResult"}
// Dependencies: {}
# [doc = " A return type for `nanosleep` and `clock_nanosleep_relative`."] # [derive (Clone)] # [must_use] pub enum NanosleepRelativeResult { # [doc = " The sleep completed normally."] Ok , # [doc = " The sleep was interrupted, the remaining time is returned."] Interrupted (Timespec) , # [doc = " An invalid time value was provided."] Err (io :: Errno) , }
};
}
