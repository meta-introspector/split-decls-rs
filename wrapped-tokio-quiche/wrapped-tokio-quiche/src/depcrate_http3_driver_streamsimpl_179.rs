// Generated macro for impl_179 (impl)
macro_rules! Depcrate_http3_driver_streamsimpl_179 {
() => {
// Module: crate::http3::driver::streams
// Provides: {"impl_179"}
// Dependencies: {}
impl Future for WaitForDownstreamData { type Output = ReceivedDownstreamData ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Self :: Output > { self . chan . as_mut () . unwrap () . poll_recv (cx) . map (| data | { ReceivedDownstreamData { stream_id : self . stream_id , chan : self . chan . take () . unwrap () , data , } }) } }
};
}
