// Generated macro for PendingClientRequest (struct)
macro_rules! Depcrate_http3_driver_clientPendingClientRequest {
() => {
// Module: crate::http3::driver::client
// Provides: {"PendingClientRequest"}
// Dependencies: {}
# [doc = " A [`PendingClientRequest`] is a request which has not yet received a"] # [doc = " response."] # [doc = ""] # [doc = " The `send` and `recv` halves are passed to the [ClientH3Controller] in an"] # [doc = " [`H3Event::IncomingHeaders`] once the server's response has been received."] struct PendingClientRequest { send : OutboundFrameSender , recv : InboundFrameStream , }
};
}
