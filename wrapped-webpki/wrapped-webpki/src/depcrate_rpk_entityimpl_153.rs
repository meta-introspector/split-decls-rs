// Generated macro for impl_153 (impl)
macro_rules! Depcrate_rpk_entityimpl_153 {
() => {
// Module: crate::rpk_entity
// Provides: {"impl_153"}
// Dependencies: {}
impl RawPublicKeyEntity < '_ > { # [doc = " Verifies the signature `signature` of message `msg` using a raw public key,"] # [doc = " supporting RFC 7250."] # [doc = ""] # [doc = " For more information on `signature_alg` and `signature` see the documentation for [`crate::end_entity::EndEntityCert::verify_signature`]."] pub fn verify_signature (& self , signature_alg : & dyn SignatureVerificationAlgorithm , msg : & [u8] , signature : & [u8] ,) -> Result < () , Error > { signed_data :: verify_signature (signature_alg , self . inner , untrusted :: Input :: from (msg) , untrusted :: Input :: from (signature) ,) } }
};
}
