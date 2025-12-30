// Generated macro for impl_522 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_522 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_522"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T : AsULE > AsRef < ZeroSlice < T > > for alloc :: vec :: Vec < T :: ULE > { fn as_ref (& self) -> & ZeroSlice < T > { ZeroSlice :: < T > :: from_ule_slice (self) } }
};
}
