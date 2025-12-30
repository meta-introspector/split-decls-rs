// Generated macro for impl_1553 (impl)
macro_rules! Depcrate_crypto_signerimpl_1553 {
() => {
// Module: crate::crypto::signer
// Provides: {"impl_1553"}
// Dependencies: {}
impl < 'a > CertificateIdentity < 'a > { # [doc = " Convert this `CertificateIdentity` into an owned version."] pub fn into_owned (self) -> CertificateIdentity < 'static > { CertificateIdentity { end_entity : self . end_entity . into_owned () , intermediates : self . intermediates . into_iter () . map (| cert | cert . into_owned ()) . collect () , } } }
};
}
