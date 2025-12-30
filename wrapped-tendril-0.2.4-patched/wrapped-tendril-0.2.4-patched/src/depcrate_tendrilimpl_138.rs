// Generated macro for impl_138 (impl)
macro_rules! Depcrate_tendrilimpl_138 {
() => {
// Module: crate::tendril
// Provides: {"impl_138"}
// Dependencies: {}
impl < F , A > PartialEq for Tendril < F , A > where F : fmt :: Format , A : Atomicity , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_byte_slice () == other . as_byte_slice () } # [inline] fn ne (& self , other : & Self) -> bool { self . as_byte_slice () != other . as_byte_slice () } }
};
}
