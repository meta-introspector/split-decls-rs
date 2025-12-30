// Generated macro for impl_437 (impl)
macro_rules! Depcrate_owned_sliceimpl_437 {
() => {
// Module: crate::owned_slice
// Provides: {"impl_437"}
// Dependencies: {}
impl Deref for OwnedSlice { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { unsafe { & * self . bytes } } }
};
}
