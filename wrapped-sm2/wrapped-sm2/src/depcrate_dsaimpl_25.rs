// Generated macro for impl_25 (impl)
macro_rules! Depcrate_dsaimpl_25 {
() => {
// Module: crate::dsa
// Provides: {"impl_25"}
// Dependencies: {}
impl TryFrom < SignatureBytes > for Signature { type Error = Error ; fn try_from (signature : SignatureBytes) -> Result < Signature > { Signature :: from_bytes (& signature) } }
};
}
