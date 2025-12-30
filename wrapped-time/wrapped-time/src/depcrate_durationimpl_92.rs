// Generated macro for impl_92 (impl)
macro_rules! Depcrate_durationimpl_92 {
() => {
// Module: crate::duration
// Provides: {"impl_92"}
// Dependencies: {}
impl Sum for Duration { # [inline] fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { iter . reduce (| a , b | a + b) . unwrap_or_default () } }
};
}
