// Generated macro for impl_69 (impl)
macro_rules! Depcrate_durationimpl_69 {
() => {
// Module: crate::duration
// Provides: {"impl_69"}
// Dependencies: {}
impl Neg for Duration { type Output = Self ; # [inline] # [track_caller] fn neg (self) -> Self :: Output { self . checked_neg () . expect ("overflow when negating duration") } }
};
}
