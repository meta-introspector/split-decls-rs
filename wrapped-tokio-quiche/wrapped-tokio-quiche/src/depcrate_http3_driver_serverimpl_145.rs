// Generated macro for impl_145 (impl)
macro_rules! Depcrate_http3_driver_serverimpl_145 {
() => {
// Module: crate::http3::driver::server
// Provides: {"impl_145"}
// Dependencies: {}
impl From < H3Event > for ServerH3Event { fn from (ev : H3Event) -> Self { match ev { H3Event :: IncomingHeaders (incoming_headers) => Self :: Headers { incoming_headers , priority : None , } , _ => Self :: Core (ev) , } } }
};
}
