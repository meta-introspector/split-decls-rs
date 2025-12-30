// Generated macro for NewClientRequest (struct)
macro_rules! Depcrate_http3_driver_clientNewClientRequest {
() => {
// Module: crate::http3::driver::client
// Provides: {"NewClientRequest"}
// Dependencies: {}
# [doc = " An HTTP request sent using a [ClientRequestSender] to the [ClientH3Driver]."] # [derive (Debug)] pub struct NewClientRequest { # [doc = " A user-defined identifier to match [`ClientH3Event::NewOutboundRequest`]"] # [doc = " to its original [`NewClientRequest`]. This ID is not used anywhere else."] pub request_id : u64 , # [doc = " The [`h3::Header`]s that make up this request."] pub headers : Vec < h3 :: Header > , # [doc = " A sender to pass the request's [`OutboundFrameSender`] to the request"] # [doc = " body."] pub body_writer : Option < oneshot :: Sender < OutboundFrameSender > > , }
};
}
