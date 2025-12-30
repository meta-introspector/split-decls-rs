// Generated macro for impl_87 (impl)
macro_rules! Depcrate_durationimpl_87 {
() => {
// Module: crate::duration
// Provides: {"impl_87"}
// Dependencies: {}
impl Div < Duration > for StdDuration { type Output = f64 ; # [inline] # [track_caller] fn div (self , rhs : Duration) -> Self :: Output { self . as_secs_f64 () / rhs . as_seconds_f64 () } }
};
}
