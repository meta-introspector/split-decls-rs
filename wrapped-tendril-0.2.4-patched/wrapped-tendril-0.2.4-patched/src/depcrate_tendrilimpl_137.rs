// Generated macro for impl_137 (impl)
macro_rules! Depcrate_tendrilimpl_137 {
() => {
// Module: crate::tendril
// Provides: {"impl_137"}
// Dependencies: {}
impl < F , A > Borrow < [u8] > for Tendril < F , A > where F : fmt :: SliceFormat , A : Atomicity , { fn borrow (& self) -> & [u8] { self . as_byte_slice () } }
};
}
