// Generated macro for impl_121 (impl)
macro_rules! Depcrate_set_header_responseimpl_121 {
() => {
// Module: crate::set_header::response
// Provides: {"impl_121"}
// Dependencies: {}
impl < ReqBody , ResBody , S , M > Service < Request < ReqBody > > for SetResponseHeader < S , M > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , M : MakeHeaderValue < Response < ResBody > > + Clone , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future , M > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { ResponseFuture { future : self . inner . call (req) , header_name : self . header_name . clone () , make : self . make . clone () , mode : self . mode , } } }
};
}
