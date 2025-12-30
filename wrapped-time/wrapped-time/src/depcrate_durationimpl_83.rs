// Generated macro for impl_83 (impl)
macro_rules! Depcrate_durationimpl_83 {
() => {
// Module: crate::duration
// Provides: {"impl_83"}
// Dependencies: {}
impl Div < f64 > for Duration { type Output = Self ; # [inline] # [track_caller] fn div (self , rhs : f64) -> Self :: Output { Self :: seconds_f64 (self . as_seconds_f64 () / rhs) } }
};
}
