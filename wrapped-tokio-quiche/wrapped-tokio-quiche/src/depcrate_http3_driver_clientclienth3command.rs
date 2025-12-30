// Generated macro for ClientH3Command (enum)
macro_rules! Depcrate_http3_driver_clientClientH3Command {
() => {
// Module: crate::http3::driver::client
// Provides: {"ClientH3Command"}
// Dependencies: {}
# [doc = " Commands accepted by [ClientH3Driver]."] # [derive (Debug)] pub enum ClientH3Command { Core (H3Command) , # [doc = " Send a new HTTP request over the [`quiche::h3::Connection`]. The driver"] # [doc = " will allocate a stream ID and report it back to the controller via"] # [doc = " [`ClientH3Event::NewOutboundRequest`]."] ClientRequest (NewClientRequest) , }
};
}
