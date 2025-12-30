// Generated macro for impl_321 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_321 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_321"}
// Dependencies: {}
impl < 'a > CertificateEntry < 'a > { pub (crate) fn new (cert : CertificateDer < 'a >) -> Self { Self { cert , extensions : CertificateExtensions :: default () , } } pub (crate) fn into_owned (self) -> CertificateEntry < 'static > { CertificateEntry { cert : self . cert . into_owned () , extensions : self . extensions . into_owned () , } } }
};
}
