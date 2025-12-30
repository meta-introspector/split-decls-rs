// Generated macro for impl_82 (impl)
macro_rules! Depcrate_durationimpl_82 {
() => {
// Module: crate::duration
// Provides: {"impl_82"}
// Dependencies: {}
impl Div < f32 > for Duration { type Output = Self ; # [inline] # [track_caller] fn div (self , rhs : f32) -> Self :: Output { Self :: seconds_f32 (self . as_seconds_f32 () / rhs) } }
};
}
