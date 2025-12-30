// Generated macro for impl_134 (impl)
macro_rules! Depcrate_msgs_deframer_handshakeimpl_134 {
() => {
// Module: crate::msgs::deframer::handshake
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a , 'b > DissectHandshakeIter < 'a , 'b > { fn new (msg : InboundPlainMessage < 'b > , containing_buffer : & 'a Locator) -> Self { Self { version : msg . version , payload : msg . payload , containing_buffer , } } }
};
}
