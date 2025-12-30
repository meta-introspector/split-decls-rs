// Generated macro for impl_353 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_353 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_353"}
// Dependencies: {}
# [doc = " RFC5246: `ClientCertificateType certificate_types<1..2^8-1>;`"] impl TlsListElement for ClientCertificateType { const SIZE_LEN : ListLength = ListLength :: NonZeroU8 { empty_error : InvalidMessage :: IllegalEmptyList ("ClientCertificateTypes") , } ; }
};
}
