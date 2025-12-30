// Generated macro for impl_313 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_313 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_313"}
// Dependencies: {}
impl < 'a > CertificateChain < 'a > { pub (crate) fn from_signer (signer : & 'a SelectedCredential) -> Self { Self (signer . identity . as_certificates () . collect () ,) } pub (crate) fn into_owned (self) -> CertificateChain < 'static > { CertificateChain (self . 0 . into_iter () . map (CertificateDer :: into_owned) . collect () ,) } }
};
}
