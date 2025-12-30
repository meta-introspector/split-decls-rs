// Generated macro for impl_182 (impl)
macro_rules! Depcrate_http3_driver_streamsimpl_182 {
() => {
// Module: crate::http3::driver::streams
// Provides: {"impl_182"}
// Dependencies: {}
impl Future for WaitForUpstreamCapacity { type Output = HaveUpstreamCapacity ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Self :: Output > { match self . chan . as_mut () . unwrap () . poll_reserve (cx) { Poll :: Ready (_) => Poll :: Ready (HaveUpstreamCapacity { stream_id : self . stream_id , chan : self . chan . take () . unwrap () , }) , Poll :: Pending => Poll :: Pending , } } }
};
}
