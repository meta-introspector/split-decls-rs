// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_timeimpl_1047 {
() => {
// Module: crate::time
// Provides: {"impl_1047"}
// Dependencies: {}
impl Add < Duration > for Time { type Output = Self ; # [doc = " Add the sub-day time of the [`Duration`] to the `Time`. Wraps on overflow."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::ext::NumericalDuration;"] # [doc = " # use time_macros::time;"] # [doc = " assert_eq!(time!(12:00) + 2.hours(), time!(14:00));"] # [doc = " assert_eq!(time!(0:00:01) + (-2).seconds(), time!(23:59:59));"] # [doc = " ```"] # [inline] fn add (self , duration : Duration) -> Self :: Output { self . adjusting_add (duration) . 1 } }
};
}
