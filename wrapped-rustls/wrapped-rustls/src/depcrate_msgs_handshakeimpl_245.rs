// Generated macro for impl_245 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_245 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_245"}
// Dependencies: {}
impl SingleProtocolName { pub (crate) fn new (single : ProtocolName) -> Self { Self (single) } const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("ProtocolNames") , } ; }
};
}
