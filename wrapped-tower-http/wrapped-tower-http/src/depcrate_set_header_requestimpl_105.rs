// Generated macro for impl_105 (impl)
macro_rules! Depcrate_set_header_requestimpl_105 {
() => {
// Module: crate::set_header::request
// Provides: {"impl_105"}
// Dependencies: {}
impl < ReqBody , ResBody , S , M > Service < Request < ReqBody > > for SetRequestHeader < S , M > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , M : MakeHeaderValue < Request < ReqBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { self . mode . apply (& self . header_name , & mut req , & mut self . make) ; self . inner . call (req) } }
};
}
