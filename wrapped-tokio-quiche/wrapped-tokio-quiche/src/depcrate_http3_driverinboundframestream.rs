// Generated macro for InboundFrameStream (type)
macro_rules! Depcrate_http3_driverInboundFrameStream {
() => {
// Module: crate::http3::driver
// Provides: {"InboundFrameStream"}
// Dependencies: {}
# [doc = " Used by a local task to receive [`InboundFrame`]s (data) on the stream or"] # [doc = " flow associated with this channel."] pub type InboundFrameStream = mpsc :: Receiver < InboundFrame > ;
};
}
