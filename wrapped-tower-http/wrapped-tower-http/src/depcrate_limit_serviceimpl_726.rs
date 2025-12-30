// Generated macro for impl_726 (impl)
macro_rules! Depcrate_limit_serviceimpl_726 {
() => {
// Module: crate::limit::service
// Provides: {"impl_726"}
// Dependencies: {}
impl < ReqBody , ResBody , S > Service < Request < ReqBody > > for RequestBodyLimit < S > where ResBody : Body , S : Service < Request < Limited < ReqBody > > , Response = Response < ResBody > > , { type Response = Response < ResponseBody < ResBody > > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let content_length = req . headers () . get (http :: header :: CONTENT_LENGTH) . and_then (| value | value . to_str () . ok () ? . parse :: < usize > () . ok ()) ; let body_limit = match content_length { Some (len) if len > self . limit => return ResponseFuture :: payload_too_large () , Some (len) => self . limit . min (len) , None => self . limit , } ; let req = req . map (| body | Limited :: new (http_body_util :: Limited :: new (body , body_limit))) ; ResponseFuture :: new (self . inner . call (req)) } }
};
}
