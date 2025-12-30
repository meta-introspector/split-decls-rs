// Generated macro for impl_100 (impl)
macro_rules! Depcrate_http_compatimpl_100 {
() => {
// Module: crate::http_compat
// Provides: {"impl_100"}
// Dependencies: {}
impl IncomingMessage for types :: Request { fn get_headers (& self) -> types :: Headers { self . get_headers () } fn consume_body (self , res : wit_bindgen :: FutureReader < Result < () , ErrorCode > > ,) -> (wit_bindgen :: StreamReader < u8 > , wit_bindgen :: FutureReader < Result < Option < types :: Trailers > , ErrorCode > > ,) { Self :: consume_body (self , res) } }
};
}
