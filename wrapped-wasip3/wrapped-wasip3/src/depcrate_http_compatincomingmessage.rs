// Generated macro for IncomingMessage (trait)
macro_rules! Depcrate_http_compatIncomingMessage {
() => {
// Module: crate::http_compat
// Provides: {"IncomingMessage"}
// Dependencies: {}
# [doc = " Internal trait representing a readable inbound HTTP message."] # [doc = ""] # [doc = " Implemented by types that expose request or response headers"] # [doc = " and provide mechanisms to consume the message body."] pub trait IncomingMessage : Unpin { fn get_headers (& self) -> types :: Headers ; fn consume_body (self , res : wit_bindgen :: FutureReader < Result < () , ErrorCode > > ,) -> (wit_bindgen :: StreamReader < u8 > , wit_bindgen :: FutureReader < Result < Option < types :: Trailers > , ErrorCode > > ,) ; }
};
}
