// Generated macro for impl_520 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_520 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_520"}
// Dependencies: {}
impl < T : AsULE + PartialOrd > PartialOrd for ZeroSlice < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
