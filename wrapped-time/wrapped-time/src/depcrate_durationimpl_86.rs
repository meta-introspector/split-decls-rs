// Generated macro for impl_86 (impl)
macro_rules! Depcrate_durationimpl_86 {
() => {
// Module: crate::duration
// Provides: {"impl_86"}
// Dependencies: {}
impl Div < StdDuration > for Duration { type Output = f64 ; # [inline] # [track_caller] fn div (self , rhs : StdDuration) -> Self :: Output { self . as_seconds_f64 () / rhs . as_secs_f64 () } }
};
}
