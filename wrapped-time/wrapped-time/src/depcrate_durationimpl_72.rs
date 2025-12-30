// Generated macro for impl_72 (impl)
macro_rules! Depcrate_durationimpl_72 {
() => {
// Module: crate::duration
// Provides: {"impl_72"}
// Dependencies: {}
impl Sub < Duration > for StdDuration { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Duration) -> Self :: Output { Duration :: try_from (self) . expect ("overflow converting `std::time::Duration` to `time::Duration`") - rhs } }
};
}
