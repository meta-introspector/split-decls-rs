// Generated macro for impl_986 (impl)
macro_rules! Depcrate_timeout_serviceimpl_986 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_986"}
// Dependencies: {}
impl < S , ReqBody > Service < Request < ReqBody > > for RequestBodyTimeout < S > where S : Service < Request < TimeoutBody < ReqBody > > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let req = req . map (| body | TimeoutBody :: new (self . timeout , body)) ; self . inner . call (req) } }
};
}
