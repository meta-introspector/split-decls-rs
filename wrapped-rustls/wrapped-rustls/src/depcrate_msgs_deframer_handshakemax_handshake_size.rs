// Generated macro for MAX_HANDSHAKE_SIZE (const)
macro_rules! Depcrate_msgs_deframer_handshakeMAX_HANDSHAKE_SIZE {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"MAX_HANDSHAKE_SIZE"}
// Dependencies: {}
# [doc = " TLS allows for handshake messages of up to 16MB.  We"] # [doc = " restrict that to 64KB to limit potential for denial-of-"] # [doc = " service."] const MAX_HANDSHAKE_SIZE : usize = 0xffff ;
};
}
