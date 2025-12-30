// Generated macro for impl_948 (impl)
macro_rules! Depcrate_set_statusimpl_948 {
() => {
// Module: crate::set_status
// Provides: {"impl_948"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for SetStatus < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { ResponseFuture { inner : self . inner . call (req) , status : Some (self . status) , } } }
};
}
