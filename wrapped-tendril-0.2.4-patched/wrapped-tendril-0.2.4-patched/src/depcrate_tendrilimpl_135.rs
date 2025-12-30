// Generated macro for impl_135 (impl)
macro_rules! Depcrate_tendrilimpl_135 {
() => {
// Module: crate::tendril
// Provides: {"impl_135"}
// Dependencies: {}
impl < F , A > Deref for Tendril < F , A > where F : fmt :: SliceFormat , A : Atomicity , { type Target = F :: Slice ; # [inline] fn deref (& self) -> & F :: Slice { unsafe { F :: Slice :: from_bytes (self . as_byte_slice ()) } } }
};
}
