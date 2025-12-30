// Generated macro for OutboundFrameStream (type)
macro_rules! Depcrate_http3_driverOutboundFrameStream {
() => {
// Module: crate::http3::driver
// Provides: {"OutboundFrameStream"}
// Dependencies: {}
# [doc = " Used internally to receive [`OutboundFrame`]s which should be sent to a peer"] # [doc = " on the stream or flow associated with this channel."] type OutboundFrameStream = mpsc :: Receiver < OutboundFrame > ;
};
}
