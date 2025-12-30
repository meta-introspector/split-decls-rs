// Generated macro for impl_85 (impl)
macro_rules! Depcrate_durationimpl_85 {
() => {
// Module: crate::duration
// Provides: {"impl_85"}
// Dependencies: {}
impl Div for Duration { type Output = f64 ; # [inline] # [track_caller] fn div (self , rhs : Self) -> Self :: Output { self . as_seconds_f64 () / rhs . as_seconds_f64 () } }
};
}
