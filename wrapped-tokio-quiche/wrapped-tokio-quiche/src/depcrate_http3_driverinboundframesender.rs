// Generated macro for InboundFrameSender (type)
macro_rules! Depcrate_http3_driverInboundFrameSender {
() => {
// Module: crate::http3::driver
// Provides: {"InboundFrameSender"}
// Dependencies: {}
# [doc = " Used internally to send [`InboundFrame`]s (data) from the peer to a local"] # [doc = " task on the stream or flow associated with this channel."] type InboundFrameSender = PollSender < InboundFrame > ;
};
}
