// Generated macro for impl_276 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_276 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_276"}
// Dependencies: {}
# [doc = " RFC8879: `CertificateCompressionAlgorithm algorithms<2..2^8-2>;`"] impl TlsListElement for CertificateCompressionAlgorithm { const SIZE_LEN : ListLength = ListLength :: NonZeroU8 { empty_error : InvalidMessage :: IllegalEmptyList ("CertificateCompressionAlgorithms") , } ; }
};
}
