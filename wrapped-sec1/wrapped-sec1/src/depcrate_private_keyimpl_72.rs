// Generated macro for impl_72 (impl)
macro_rules! Depcrate_private_keyimpl_72 {
() => {
// Module: crate::private_key
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for EcPrivateKey < 'a > { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < EcPrivateKey < 'a > > { Ok (Self :: from_der (bytes) ?) } }
};
}
