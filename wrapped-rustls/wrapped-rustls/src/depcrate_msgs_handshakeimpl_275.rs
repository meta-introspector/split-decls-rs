// Generated macro for impl_275 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_275 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_275"}
// Dependencies: {}
# [doc = " RFC7250: `CertificateType client_certificate_types<1..2^8-1>;`"] # [doc = ""] # [doc = " Ditto `CertificateType server_certificate_types<1..2^8-1>;`"] impl TlsListElement for CertificateType { const SIZE_LEN : ListLength = ListLength :: NonZeroU8 { empty_error : InvalidMessage :: IllegalEmptyList ("CertificateTypes") , } ; }
};
}
