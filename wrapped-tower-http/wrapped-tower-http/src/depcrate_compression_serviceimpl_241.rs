// Generated macro for impl_241 (impl)
macro_rules! Depcrate_compression_serviceimpl_241 {
() => {
// Module: crate::compression::service
// Provides: {"impl_241"}
// Dependencies: {}
impl < ReqBody , ResBody , S , P > Service < Request < ReqBody > > for Compression < S , P > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , ResBody : Body , P : Predicate , { type Response = Response < CompressionBody < ResBody > > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future , P > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let encoding = Encoding :: from_headers (req . headers () , self . accept) ; ResponseFuture { inner : self . inner . call (req) , encoding , predicate : self . predicate . clone () , quality : self . quality , } } }
};
}
