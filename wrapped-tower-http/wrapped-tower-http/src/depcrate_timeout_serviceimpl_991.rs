// Generated macro for impl_991 (impl)
macro_rules! Depcrate_timeout_serviceimpl_991 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_991"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for ResponseBodyTimeout < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = Response < TimeoutBody < ResBody > > ; type Error = S :: Error ; type Future = ResponseBodyTimeoutFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { ResponseBodyTimeoutFuture { inner : self . inner . call (req) , timeout : self . timeout , } } }
};
}
