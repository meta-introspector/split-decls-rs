// Generated macro for impl_21 (impl)
macro_rules! Depcrate_dsaimpl_21 {
() => {
// Module: crate::dsa
// Provides: {"impl_21"}
// Dependencies: {}
impl Debug for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "sm2::dsa::Signature(") ? ; for byte in self . to_bytes () { write ! (f , "{byte:02X}") ? ; } write ! (f , ")") } }
};
}
