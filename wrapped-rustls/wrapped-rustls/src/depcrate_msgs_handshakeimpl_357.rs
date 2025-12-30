// Generated macro for impl_357 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_357 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_357"}
// Dependencies: {}
# [doc = " RFC8446: `DistinguishedName authorities<3..2^16-1>;` however,"] # [doc = " RFC5246: `DistinguishedName certificate_authorities<0..2^16-1>;`"] impl TlsListElement for DistinguishedName { const SIZE_LEN : ListLength = ListLength :: U16 ; }
};
}
