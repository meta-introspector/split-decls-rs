// Generated macro for impl_71 (impl)
macro_rules! Depcrate_durationimpl_71 {
() => {
// Module: crate::duration
// Provides: {"impl_71"}
// Dependencies: {}
impl Sub < StdDuration > for Duration { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : StdDuration) -> Self :: Output { self - Self :: try_from (rhs) . expect ("overflow converting `std::time::Duration` to `time::Duration`") } }
};
}
