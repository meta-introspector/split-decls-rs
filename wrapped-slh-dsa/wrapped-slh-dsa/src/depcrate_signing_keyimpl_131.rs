// Generated macro for impl_131 (impl)
macro_rules! Depcrate_signing_keyimpl_131 {
() => {
// Module: crate::signing_key
// Provides: {"impl_131"}
// Dependencies: {}
impl < P : ParameterSet > MultipartSigner < Signature < P > > for SigningKey < P > { fn try_multipart_sign (& self , msg : & [& [u8]]) -> Result < Signature < P > , Error > { self . raw_try_sign_with_context (msg , & [] , None) } }
};
}
