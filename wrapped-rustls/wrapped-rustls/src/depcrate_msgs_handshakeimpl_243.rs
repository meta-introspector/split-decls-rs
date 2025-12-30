// Generated macro for impl_243 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_243 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_243"}
// Dependencies: {}
# [doc = " RFC7301: `ProtocolName protocol_name_list<2..2^16-1>`"] impl TlsListElement for ProtocolName { const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("ProtocolNames") , } ; }
};
}
