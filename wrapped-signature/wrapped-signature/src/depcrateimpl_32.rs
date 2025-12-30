// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Signature { type Error = < [u8 ; SIGNATURE_BYTES] as TryFrom < & 'a [u8] > > :: Error ; # [inline] fn try_from (signature : & 'a [u8]) -> Result < Self , Self :: Error > { < [u8 ; SIGNATURE_BYTES] > :: try_from (signature) . map (Self :: from) } }
};
}
