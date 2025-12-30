// Generated macro for impl_254 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_254 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_254"}
// Dependencies: {}
# [doc = " RFC8446: `PskIdentity identities<7..2^16-1>;`"] impl TlsListElement for PresharedKeyIdentity { const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("PskIdentities") , } ; }
};
}
