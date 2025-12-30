// Generated macro for impl_1079 (impl)
macro_rules! Depcrate_services_redirectimpl_1079 {
() => {
// Module: crate::services::redirect
// Provides: {"impl_1079"}
// Dependencies: {}
impl < R , ResBody > Service < R > for Redirect < ResBody > where ResBody : Default , { type Response = Response < ResBody > ; type Error = Infallible ; type Future = ResponseFuture < ResBody > ; # [inline] fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn call (& mut self , _req : R) -> Self :: Future { ResponseFuture { status_code : self . status_code , location : Some (self . location . clone ()) , _marker : PhantomData , } } }
};
}
