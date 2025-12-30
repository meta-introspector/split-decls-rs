// Generated macro for impl_322 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_322 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_322"}
// Dependencies: {}
impl TlsListElement for CertificateEntry < '_ > { const SIZE_LEN : ListLength = ListLength :: U24 { max : CERTIFICATE_MAX_SIZE_LIMIT , error : InvalidMessage :: CertificatePayloadTooLarge , } ; }
};
}
