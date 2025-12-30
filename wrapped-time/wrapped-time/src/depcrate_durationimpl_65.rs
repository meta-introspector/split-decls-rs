// Generated macro for impl_65 (impl)
macro_rules! Depcrate_durationimpl_65 {
() => {
// Module: crate::duration
// Provides: {"impl_65"}
// Dependencies: {}
impl Add < StdDuration > for Duration { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , std_duration : StdDuration) -> Self :: Output { self + Self :: try_from (std_duration) . expect ("overflow converting `std::time::Duration` to `time::Duration`") } }
};
}
