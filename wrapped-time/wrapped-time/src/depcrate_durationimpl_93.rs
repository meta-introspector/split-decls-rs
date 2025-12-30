// Generated macro for impl_93 (impl)
macro_rules! Depcrate_durationimpl_93 {
() => {
// Module: crate::duration
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a > Sum < & 'a Self > for Duration { # [inline] fn sum < I : Iterator < Item = & 'a Self > > (iter : I) -> Self { iter . copied () . sum () } }
};
}
