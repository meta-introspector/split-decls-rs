// Generated macro for impl_136 (impl)
macro_rules! Depcrate_tendrilimpl_136 {
() => {
// Module: crate::tendril
// Provides: {"impl_136"}
// Dependencies: {}
impl < F , A > DerefMut for Tendril < F , A > where F : fmt :: SliceFormat , A : Atomicity , { # [inline] fn deref_mut (& mut self) -> & mut F :: Slice { unsafe { F :: Slice :: from_mut_bytes (self . as_mut_byte_slice ()) } } }
};
}
