// Generated macro for impl_179 (impl)
macro_rules! Depcrate_verifying_keyimpl_179 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_179"}
// Dependencies: {}
impl < P : ParameterSet > MultipartVerifier < Signature < P > > for VerifyingKey < P > { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature < P >) -> Result < () , Error > { self . raw_try_verify_with_context (msg , & [] , signature) } }
};
}
