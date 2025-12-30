// Generated macro for impl_317 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_317 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_317"}
// Dependencies: {}
impl CertificateExtensions < '_ > { fn into_owned (self) -> CertificateExtensions < 'static > { CertificateExtensions { status : self . status . map (| s | s . into_owned ()) , } } }
};
}
