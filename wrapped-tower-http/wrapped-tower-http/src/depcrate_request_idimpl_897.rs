// Generated macro for impl_897 (impl)
macro_rules! Depcrate_request_idimpl_897 {
() => {
// Module: crate::request_id
// Provides: {"impl_897"}
// Dependencies: {}
impl < S , M , ReqBody , ResBody > Service < Request < ReqBody > > for SetRequestId < S , M > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , M : MakeRequestId , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { if let Some (request_id) = req . headers () . get (& self . header_name) { if req . extensions () . get :: < RequestId > () . is_none () { let request_id = request_id . clone () ; req . extensions_mut () . insert (RequestId :: new (request_id)) ; } } else if let Some (request_id) = self . make_request_id . make_request_id (& req) { req . extensions_mut () . insert (request_id . clone ()) ; req . headers_mut () . insert (self . header_name . clone () , request_id . 0) ; } self . inner . call (req) } }
};
}
