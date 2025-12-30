// Generated macro for impl_272 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_272 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_272"}
// Dependencies: {}
impl SupportedProtocolVersions { # [doc = " Return true if `filter` returns true for any enabled version."] pub (crate) fn any (& self , filter : impl Fn (ProtocolVersion) -> bool) -> bool { if self . tls13 && filter (ProtocolVersion :: TLSv1_3) { return true ; } if self . tls12 && filter (ProtocolVersion :: TLSv1_2) { return true ; } false } const LIST_LENGTH : ListLength = ListLength :: NonZeroU8 { empty_error : InvalidMessage :: IllegalEmptyList ("ProtocolVersions") , } ; }
};
}
