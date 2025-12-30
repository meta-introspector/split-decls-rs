// Generated macro for impl_79 (impl)
macro_rules! Depcrate_durationimpl_79 {
() => {
// Module: crate::duration
// Provides: {"impl_79"}
// Dependencies: {}
impl Mul < f64 > for Duration { type Output = Self ; # [inline] # [track_caller] fn mul (self , rhs : f64) -> Self :: Output { Self :: seconds_f64 (self . as_seconds_f64 () * rhs) } }
};
}
