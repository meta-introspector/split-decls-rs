// Generated macro for impl_59 (impl)
macro_rules! Depcrate_arcimpl_59 {
() => {
// Module: crate::arc
// Provides: {"impl_59"}
// Dependencies: {}
impl < A > FromIterator < A > for Arc < [A] > { fn from_iter < T : IntoIterator < Item = A > > (iter : T) -> Self { UniqueArc :: from_iter (iter) . shareable () } }
};
}
