// Generated macro for impl_385 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_385 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_385"}
// Dependencies: {}
# [doc = " draft-ietf-tls-esni-24: `HpkeSymmetricCipherSuite cipher_suites<4..2^16-4>;`"] impl TlsListElement for HpkeSymmetricCipherSuite { const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("HpkeSymmetricCipherSuites") , } ; }
};
}
