// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_timeimpl_1048 {
() => {
// Module: crate::time
// Provides: {"impl_1048"}
// Dependencies: {}
impl Add < StdDuration > for Time { type Output = Self ; # [doc = " Add the sub-day time of the [`std::time::Duration`] to the `Time`. Wraps on overflow."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::ext::NumericalStdDuration;"] # [doc = " # use time_macros::time;"] # [doc = " assert_eq!(time!(12:00) + 2.std_hours(), time!(14:00));"] # [doc = " assert_eq!(time!(23:59:59) + 2.std_seconds(), time!(0:00:01));"] # [doc = " ```"] # [inline] fn add (self , duration : StdDuration) -> Self :: Output { self . adjusting_add_std (duration) . 1 } }
};
}
