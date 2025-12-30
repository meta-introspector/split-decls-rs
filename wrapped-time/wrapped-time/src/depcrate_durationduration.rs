// Generated macro for Duration (struct)
macro_rules! Depcrate_durationDuration {
() => {
// Module: crate::duration
// Provides: {"Duration"}
// Dependencies: {}
# [doc = " A span of time with nanosecond precision."] # [doc = ""] # [doc = " Each `Duration` is composed of a whole number of seconds and a fractional part represented in"] # [doc = " nanoseconds."] # [doc = ""] # [doc = " This implementation allows for negative durations, unlike [`core::time::Duration`]."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct Duration { # [doc = " Number of whole seconds."] seconds : i64 , # [doc = " Number of nanoseconds within the second. The sign always matches the `seconds` field."] nanoseconds : Nanoseconds , padding : Padding , }
};
}
