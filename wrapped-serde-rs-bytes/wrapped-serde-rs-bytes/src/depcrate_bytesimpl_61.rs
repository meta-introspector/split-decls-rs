// Generated macro for impl_61 (impl)
macro_rules! Depcrate_bytesimpl_61 {
() => {
// Module: crate::bytes
// Provides: {"impl_61"}
// Dependencies: {}
impl < Rhs > PartialOrd < Rhs > for Bytes where Rhs : ? Sized + AsRef < [u8] > , { fn partial_cmp (& self , other : & Rhs) -> Option < Ordering > { self . as_ref () . partial_cmp (other . as_ref ()) } }
};
}
