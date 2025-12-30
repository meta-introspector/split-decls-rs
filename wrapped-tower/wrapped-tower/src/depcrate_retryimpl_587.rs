// Generated macro for impl_587 (impl)
macro_rules! Depcrate_retryimpl_587 {
() => {
// Module: crate::retry
// Provides: {"impl_587"}
// Dependencies: {}
impl < P , S , Request > Service < Request > for Retry < P , S > where P : Policy < Request , S :: Response , S :: Error > + Clone , S : Service < Request > + Clone , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < P , S , Request > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . service . poll_ready (cx) } fn call (& mut self , request : Request) -> Self :: Future { let cloned = self . policy . clone_request (& request) ; let future = self . service . call (request) ; ResponseFuture :: new (cloned , self . clone () , future) } }
};
}
