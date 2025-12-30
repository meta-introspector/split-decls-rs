// Generated macro for impl_26 (impl)
macro_rules! Depcrate_dsaimpl_26 {
() => {
// Module: crate::dsa
// Provides: {"impl_26"}
// Dependencies: {}
impl TryFrom < & SignatureBytes > for Signature { type Error = Error ; fn try_from (signature : & SignatureBytes) -> Result < Signature > { Signature :: from_bytes (signature) } }
};
}
