// Generated macro for impl_130 (impl)
macro_rules! Depcrate_signing_keyimpl_130 {
() => {
// Module: crate::signing_key
// Provides: {"impl_130"}
// Dependencies: {}
impl < P : ParameterSet > Signer < Signature < P > > for SigningKey < P > { fn try_sign (& self , msg : & [u8]) -> Result < Signature < P > , Error > { self . try_multipart_sign (& [msg]) } }
};
}
