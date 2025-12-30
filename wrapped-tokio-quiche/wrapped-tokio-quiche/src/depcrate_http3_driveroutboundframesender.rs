// Generated macro for OutboundFrameSender (type)
macro_rules! Depcrate_http3_driverOutboundFrameSender {
() => {
// Module: crate::http3::driver
// Provides: {"OutboundFrameSender"}
// Dependencies: {}
# [doc = " Used by a local task to send [`OutboundFrame`]s to a peer on the"] # [doc = " stream or flow associated with this channel."] pub type OutboundFrameSender = PollSender < OutboundFrame > ;
};
}
