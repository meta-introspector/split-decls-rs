// Generated macro for impl_1542 (impl)
macro_rules! Depcrate_crypto_signerimpl_1542 {
() => {
// Module: crate::crypto::signer
// Provides: {"impl_1542"}
// Dependencies: {}
impl ClientCredentialResolver for SingleCredential { fn resolve (& self , request : & CredentialRequest < '_ >) -> Option < SelectedCredential > { match (& * self . credentials . identity , request . negotiated_type ()) { (Identity :: X509 (_) , CertificateType :: X509) | (Identity :: RawPublicKey (_) , CertificateType :: RawPublicKey) => self . credentials . signer (request . signature_schemes ()) , _ => None , } } fn supported_certificate_types (& self) -> & 'static [CertificateType] { self . types } }
};
}
