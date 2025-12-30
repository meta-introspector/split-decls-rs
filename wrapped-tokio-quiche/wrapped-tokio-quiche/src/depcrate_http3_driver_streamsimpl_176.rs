// Generated macro for impl_176 (impl)
macro_rules! Depcrate_http3_driver_streamsimpl_176 {
() => {
// Module: crate::http3::driver::streams
// Provides: {"impl_176"}
// Dependencies: {}
impl Future for WaitForStream { type Output = StreamReady ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . get_mut () { WaitForStream :: Downstream (d) => Pin :: new (d) . poll (cx) . map (StreamReady :: Downstream) , WaitForStream :: Upstream (u) => Pin :: new (u) . poll (cx) . map (StreamReady :: Upstream) , } } }
};
}
