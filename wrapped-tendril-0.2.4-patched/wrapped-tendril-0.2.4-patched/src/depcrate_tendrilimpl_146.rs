// Generated macro for impl_146 (impl)
macro_rules! Depcrate_tendrilimpl_146 {
() => {
// Module: crate::tendril
// Provides: {"impl_146"}
// Dependencies: {}
impl < F , A > Tendril < F , A > where F : fmt :: SliceFormat , A : Atomicity , { # [doc = " Build a `Tendril` by copying a slice."] # [inline] pub fn from_slice (x : & F :: Slice) -> Tendril < F , A > { unsafe { Tendril :: from_byte_slice_without_validating (x . as_bytes ()) } } # [doc = " Push a slice onto the end of the `Tendril`."] # [inline] pub fn push_slice (& mut self , x : & F :: Slice) { unsafe { self . push_bytes_without_validating (x . as_bytes ()) } } }
};
}
