// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_timeimpl_1050 {
() => {
// Module: crate::time
// Provides: {"impl_1050"}
// Dependencies: {}
impl Sub < Duration > for Time { type Output = Self ; # [doc = " Subtract the sub-day time of the [`Duration`] from the `Time`. Wraps on overflow."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::ext::NumericalDuration;"] # [doc = " # use time_macros::time;"] # [doc = " assert_eq!(time!(14:00) - 2.hours(), time!(12:00));"] # [doc = " assert_eq!(time!(23:59:59) - (-2).seconds(), time!(0:00:01));"] # [doc = " ```"] # [inline] fn sub (self , duration : Duration) -> Self :: Output { self . adjusting_sub (duration) . 1 } }
};
}
