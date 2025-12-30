// Generated macro for impl_101 (impl)
macro_rules! Depcrate_http_compatimpl_101 {
() => {
// Module: crate::http_compat
// Provides: {"impl_101"}
// Dependencies: {}
impl IncomingMessage for types :: Response { fn get_headers (& self) -> types :: Headers { self . get_headers () } fn consume_body (self , res : wit_bindgen :: FutureReader < Result < () , ErrorCode > > ,) -> (wit_bindgen :: StreamReader < u8 > , wit_bindgen :: FutureReader < Result < Option < types :: Trailers > , ErrorCode > > ,) { Self :: consume_body (self , res) } }
};
}
