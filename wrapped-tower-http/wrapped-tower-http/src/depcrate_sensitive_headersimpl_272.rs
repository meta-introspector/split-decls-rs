// Generated macro for impl_272 (impl)
macro_rules! Depcrate_sensitive_headersimpl_272 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_272"}
// Dependencies: {}
impl < ReqBody , ResBody , S > Service < Request < ReqBody > > for SetSensitiveRequestHeaders < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { let headers = req . headers_mut () ; for header in & * self . headers { if let http :: header :: Entry :: Occupied (mut entry) = headers . entry (header) { for value in entry . iter_mut () { value . set_sensitive (true) ; } } } self . inner . call (req) } }
};
}
