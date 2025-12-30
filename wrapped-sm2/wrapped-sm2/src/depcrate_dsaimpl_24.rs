// Generated macro for impl_24 (impl)
macro_rules! Depcrate_dsaimpl_24 {
() => {
// Module: crate::dsa
// Provides: {"impl_24"}
// Dependencies: {}
impl SignatureEncoding for Signature { type Repr = SignatureBytes ; fn to_bytes (& self) -> Self :: Repr { self . into () } fn encoded_len (& self) -> usize { Self :: BYTE_SIZE } }
};
}
