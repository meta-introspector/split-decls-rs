// Generated macro for impl_147 (impl)
macro_rules! Depcrate_propagate_headerimpl_147 {
() => {
// Module: crate::propagate_header
// Provides: {"impl_147"}
// Dependencies: {}
impl < ReqBody , ResBody , S > Service < Request < ReqBody > > for PropagateHeader < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let value = req . headers () . get (& self . header) . cloned () ; ResponseFuture { future : self . inner . call (req) , header_and_value : Some (self . header . clone ()) . zip (value) , } } }
};
}
