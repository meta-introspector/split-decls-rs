// Generated macro for impl_173 (impl)
macro_rules! Depcrate_http3_driver_streamsimpl_173 {
() => {
// Module: crate::http3::driver::streams
// Provides: {"impl_173"}
// Dependencies: {}
impl FlowCtx { # [doc = " Creates a new [FlowCtx]. This method returns the context itself"] # [doc = " as well as the datagram receiver for this flow."] pub (crate) fn new (capacity : usize) -> (Self , InboundFrameStream) { let (forward_sender , forward_receiver) = mpsc :: channel (capacity) ; let ctx = FlowCtx { send : forward_sender , } ; (ctx , forward_receiver) } # [doc = " Tries to send a datagram to the flow receiver, but drops it if the"] # [doc = " channel is full."] pub (crate) fn send_best_effort (& self , datagram : InboundFrame) { let _ = self . send . try_send (datagram) ; } }
};
}
