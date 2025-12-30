// Generated macro for impl_233 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_233 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_233"}
// Dependencies: {}
# [doc = " RFC8446: `SignatureScheme supported_signature_algorithms<2..2^16-2>;`"] impl TlsListElement for SignatureScheme { const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: NoSignatureSchemes , } ; }
};
}
