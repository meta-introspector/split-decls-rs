// Generated macro for impl_178 (impl)
macro_rules! Depcrate_verifying_keyimpl_178 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_178"}
// Dependencies: {}
impl < P : ParameterSet > Verifier < Signature < P > > for VerifyingKey < P > { fn verify (& self , msg : & [u8] , signature : & Signature < P >) -> Result < () , Error > { self . multipart_verify (& [msg] , signature) } }
};
}
