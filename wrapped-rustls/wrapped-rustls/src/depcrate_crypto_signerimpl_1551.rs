// Generated macro for impl_1551 (impl)
macro_rules! Depcrate_crypto_signerimpl_1551 {
() => {
// Module: crate::crypto::signer
// Provides: {"impl_1551"}
// Dependencies: {}
impl < 'a , C , R > Iterator for IdentityCertificateIterator < C , R > where C : Iterator < Item = CertificateDer < 'a > > , R : Iterator < Item = CertificateDer < 'a > > , { type Item = CertificateDer < 'a > ; fn next (& mut self) -> Option < Self :: Item > { match self { Self :: X509 (iter) => iter . next () , Self :: RawPublicKey (iter) => iter . next () , } } }
};
}
