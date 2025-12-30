// Generated macro for impl_513 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_513 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_513"}
// Dependencies: {}
unsafe impl < T : AsULE + 'static > VarULE for ZeroSlice < T > { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { T :: ULE :: validate_bytes (bytes) } # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { Self :: from_ule_slice (T :: ULE :: slice_from_bytes_unchecked (bytes)) } }
};
}
