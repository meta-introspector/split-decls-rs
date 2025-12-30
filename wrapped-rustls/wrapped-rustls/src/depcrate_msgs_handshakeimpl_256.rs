// Generated macro for impl_256 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_256 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_256"}
// Dependencies: {}
# [doc = " RFC8446: `PskBinderEntry binders<33..2^16-1>;`"] impl TlsListElement for PresharedKeyBinder { const SIZE_LEN : ListLength = ListLength :: NonZeroU16 { empty_error : InvalidMessage :: IllegalEmptyList ("PskBinders") , } ; }
};
}
