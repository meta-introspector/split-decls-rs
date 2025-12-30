// Generated macro for impl_278 (impl)
macro_rules! Depcrate_sensitive_headersimpl_278 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_278"}
// Dependencies: {}
impl < ReqBody , ResBody , S > Service < Request < ReqBody > > for SetSensitiveResponseHeaders < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = SetSensitiveResponseHeadersResponseFuture < S :: Future > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { SetSensitiveResponseHeadersResponseFuture { future : self . inner . call (req) , headers : self . headers . clone () , } } }
};
}
