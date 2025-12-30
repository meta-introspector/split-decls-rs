// Generated macro for impl_978 (impl)
macro_rules! Depcrate_timeout_serviceimpl_978 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_978"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for Timeout < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , ResBody : Default , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let sleep = tokio :: time :: sleep (self . timeout) ; ResponseFuture { inner : self . inner . call (req) , sleep , status_code : self . status_code , } } }
};
}
