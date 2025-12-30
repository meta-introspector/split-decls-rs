// Generated macro for impl_384 (impl)
macro_rules! Depcrate_decompression_serviceimpl_384 {
() => {
// Module: crate::decompression::service
// Provides: {"impl_384"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for Decompression < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , ResBody : Body , { type Response = Response < DecompressionBody < ResBody > > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { if let header :: Entry :: Vacant (entry) = req . headers_mut () . entry (ACCEPT_ENCODING) { if let Some (accept) = self . accept . to_header_value () { entry . insert (accept) ; } } ResponseFuture { inner : self . inner . call (req) , accept : self . accept , } } }
};
}
