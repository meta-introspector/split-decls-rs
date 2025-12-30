// Generated macro for impl_27 (impl)
macro_rules! Depcrate_dsaimpl_27 {
() => {
// Module: crate::dsa
// Provides: {"impl_27"}
// Dependencies: {}
impl TryFrom < & [u8] > for Signature { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Signature > { Signature :: from_slice (bytes) } }
};
}
