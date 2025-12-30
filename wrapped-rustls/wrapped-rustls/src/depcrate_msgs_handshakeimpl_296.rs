// Generated macro for impl_296 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_296 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_296"}
// Dependencies: {}
# [doc = " RFC8446: `CipherSuite cipher_suites<2..2^16-2>;`"] impl TlsListElement for CipherSuite { const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("CipherSuites") , } ; }
};
}
