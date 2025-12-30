// Generated macro for impl_1271 (impl)
macro_rules! Depcrate_validate_requestimpl_1271 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1271"}
// Dependencies: {}
impl < ReqBody , ResBody , S , V > Service < Request < ReqBody > > for ValidateRequestHeader < S , V > where V : ValidateRequest < ReqBody , ResponseBody = ResBody > , S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = Response < ResBody > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future , ResBody > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { match self . validate . validate (& mut req) { Ok (_) => ResponseFuture :: future (self . inner . call (req)) , Err (res) => ResponseFuture :: invalid_header_value (res) , } } }
};
}
