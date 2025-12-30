// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bytesimpl_60 {
() => {
// Module: crate::bytes
// Provides: {"impl_60"}
// Dependencies: {}
impl < Rhs > PartialEq < Rhs > for Bytes where Rhs : ? Sized + AsRef < [u8] > , { fn eq (& self , other : & Rhs) -> bool { self . as_ref () . eq (other . as_ref ()) } }
};
}
