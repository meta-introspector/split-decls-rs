// Generated macro for impl_903 (impl)
macro_rules! Depcrate_request_idimpl_903 {
() => {
// Module: crate::request_id
// Provides: {"impl_903"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for PropagateRequestId < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = PropagateRequestIdResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let request_id = req . headers () . get (& self . header_name) . cloned () . map (RequestId :: new) ; PropagateRequestIdResponseFuture { inner : self . inner . call (req) , header_name : self . header_name . clone () , request_id , } } }
};
}
