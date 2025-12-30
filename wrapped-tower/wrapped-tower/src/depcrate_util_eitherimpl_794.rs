// Generated macro for impl_794 (impl)
macro_rules! Depcrate_util_eitherimpl_794 {
() => {
// Module: crate::util::either
// Provides: {"impl_794"}
// Dependencies: {}
impl < A , B , Request > Service < Request > for Either < A , B > where A : Service < Request > , B : Service < Request , Response = A :: Response , Error = A :: Error > , { type Response = A :: Response ; type Error = A :: Error ; type Future = EitherResponseFuture < A :: Future , B :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self { Either :: Left (service) => service . poll_ready (cx) , Either :: Right (service) => service . poll_ready (cx) , } } fn call (& mut self , request : Request) -> Self :: Future { match self { Either :: Left (service) => EitherResponseFuture { kind : Kind :: Left { inner : service . call (request) , } , } , Either :: Right (service) => EitherResponseFuture { kind : Kind :: Right { inner : service . call (request) , } , } , } } }
};
}
