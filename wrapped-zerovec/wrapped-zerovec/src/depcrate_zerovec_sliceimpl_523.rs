// Generated macro for impl_523 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_523 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_523"}
// Dependencies: {}
impl < T : AsULE > AsRef < ZeroSlice < T > > for & [T :: ULE] { fn as_ref (& self) -> & ZeroSlice < T > { ZeroSlice :: < T > :: from_ule_slice (self) } }
};
}
