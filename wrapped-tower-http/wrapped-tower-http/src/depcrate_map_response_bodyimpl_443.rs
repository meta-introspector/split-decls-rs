// Generated macro for impl_443 (impl)
macro_rules! Depcrate_map_response_bodyimpl_443 {
() => {
// Module: crate::map_response_body
// Provides: {"impl_443"}
// Dependencies: {}
impl < F , S , ReqBody , ResBody , NewResBody > Service < Request < ReqBody > > for MapResponseBody < S , F > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , F : FnMut (ResBody) -> NewResBody + Clone , { type Response = Response < NewResBody > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future , F > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { ResponseFuture { inner : self . inner . call (req) , f : self . f . clone () , } } }
};
}
