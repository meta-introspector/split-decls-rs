// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_timeimpl_1051 {
() => {
// Module: crate::time
// Provides: {"impl_1051"}
// Dependencies: {}
impl Sub < StdDuration > for Time { type Output = Self ; # [doc = " Subtract the sub-day time of the [`std::time::Duration`] from the `Time`. Wraps on overflow."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::ext::NumericalStdDuration;"] # [doc = " # use time_macros::time;"] # [doc = " assert_eq!(time!(14:00) - 2.std_hours(), time!(12:00));"] # [doc = " assert_eq!(time!(0:00:01) - 2.std_seconds(), time!(23:59:59));"] # [doc = " ```"] # [inline] fn sub (self , duration : StdDuration) -> Self :: Output { self . adjusting_sub_std (duration) . 1 } }
};
}
