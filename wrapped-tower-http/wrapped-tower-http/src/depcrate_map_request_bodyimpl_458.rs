// Generated macro for impl_458 (impl)
macro_rules! Depcrate_map_request_bodyimpl_458 {
() => {
// Module: crate::map_request_body
// Provides: {"impl_458"}
// Dependencies: {}
impl < F , S , ReqBody , ResBody , NewReqBody > Service < Request < ReqBody > > for MapRequestBody < S , F > where S : Service < Request < NewReqBody > , Response = Response < ResBody > > , F : FnMut (ReqBody) -> NewReqBody , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let req = req . map (& mut self . f) ; self . inner . call (req) } }
};
}
