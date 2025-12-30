// Generated macro for impl_809 (impl)
macro_rules! Depcrate_util_future_serviceimpl_809 {
() => {
// Module: crate::util::future_service
// Provides: {"impl_809"}
// Dependencies: {}
impl < F , S , R , E > Service < R > for FutureService < F , S > where F : Future < Output = Result < S , E > > + Unpin , S : Service < R , Error = E > , { type Response = S :: Response ; type Error = E ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { loop { self . state = match & mut self . state { State :: Future (fut) => { let fut = Pin :: new (fut) ; let svc = std :: task :: ready ! (fut . poll (cx) ?) ; State :: Service (svc) } State :: Service (svc) => return svc . poll_ready (cx) , } ; } } fn call (& mut self , req : R) -> Self :: Future { if let State :: Service (svc) = & mut self . state { svc . call (req) } else { panic ! ("FutureService::call was called before FutureService::poll_ready") } } }
};
}
