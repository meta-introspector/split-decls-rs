// Generated macro for impl_297 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_297 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_297"}
// Dependencies: {}
# [doc = " RFC5246: `CompressionMethod compression_methods<1..2^8-1>;`"] impl TlsListElement for Compression { const SIZE_LEN : ListLength = ListLength :: NonZeroU8 { empty_error : InvalidMessage :: IllegalEmptyList ("Compressions") , } ; }
};
}
