// Generated macro for impl_1541 (impl)
macro_rules! Depcrate_crypto_signerimpl_1541 {
() => {
// Module: crate::crypto::signer
// Provides: {"impl_1541"}
// Dependencies: {}
impl From < Credentials > for SingleCredential { fn from (credentials : Credentials) -> Self { match & * credentials . identity { Identity :: X509 (_) => Self { credentials , types : & [CertificateType :: X509] , } , Identity :: RawPublicKey (_) => Self { credentials , types : & [CertificateType :: RawPublicKey] , } , } } }
};
}
