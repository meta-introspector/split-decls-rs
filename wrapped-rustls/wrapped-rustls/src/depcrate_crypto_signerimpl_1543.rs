// Generated macro for impl_1543 (impl)
macro_rules! Depcrate_crypto_signerimpl_1543 {
() => {
// Module: crate::crypto::signer
// Provides: {"impl_1543"}
// Dependencies: {}
impl ServerCredentialResolver for SingleCredential { fn resolve (& self , client_hello : & ClientHello < '_ >) -> Result < SelectedCredential , Error > { self . credentials . signer (client_hello . signature_schemes ()) . ok_or (Error :: PeerIncompatible (PeerIncompatible :: NoSignatureSchemesInCommon ,)) } fn supported_certificate_types (& self) -> & 'static [CertificateType] { self . types } }
};
}
