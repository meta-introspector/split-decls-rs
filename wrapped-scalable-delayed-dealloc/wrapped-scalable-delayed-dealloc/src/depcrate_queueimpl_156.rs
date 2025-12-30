// Generated macro for impl_156 (impl)
macro_rules! Depcrate_queueimpl_156 {
() => {
// Module: crate::queue
// Provides: {"impl_156"}
// Dependencies: {}
impl < T : 'static > FromIterator < T > for Queue < T > { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let into_iter = iter . into_iter () ; let queue = Self :: default () ; into_iter . for_each (| v | { queue . push (v) ; }) ; queue } }
};
}
