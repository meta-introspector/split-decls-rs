// Generated macro for impl_77 (impl)
macro_rules! Depcrate_durationimpl_77 {
() => {
// Module: crate::duration
// Provides: {"impl_77"}
// Dependencies: {}
impl Mul < f32 > for Duration { type Output = Self ; # [inline] # [track_caller] fn mul (self , rhs : f32) -> Self :: Output { Self :: seconds_f32 (self . as_seconds_f32 () * rhs) } }
};
}
