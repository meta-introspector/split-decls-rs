// Generated macro for CERTIFICATE_MAX_SIZE_LIMIT (const)
macro_rules! Depcrate_msgs_codecCERTIFICATE_MAX_SIZE_LIMIT {
() => {
// Module: crate::msgs::codec
// Provides: {"CERTIFICATE_MAX_SIZE_LIMIT"}
// Dependencies: {}
# [doc = " TLS has a 16MB size limit on any handshake message,"] # [doc = " plus a 16MB limit on any given certificate."] # [doc = ""] # [doc = " We contract that to 64KB to limit the amount of memory allocation"] # [doc = " that is directly controllable by the peer."] pub (crate) const CERTIFICATE_MAX_SIZE_LIMIT : usize = 0x1_0000 ;
};
}
