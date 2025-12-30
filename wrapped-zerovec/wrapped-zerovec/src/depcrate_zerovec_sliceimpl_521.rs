// Generated macro for impl_521 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_521 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_521"}
// Dependencies: {}
impl < T : AsULE + Ord > Ord for ZeroSlice < T > { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
