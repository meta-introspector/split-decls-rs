// Generated macro for impl_294 (impl)
macro_rules! Depcrate_limit_concurrency_serviceimpl_294 {
() => {
// Module: crate::limit::concurrency::service
// Provides: {"impl_294"}
// Dependencies: {}
impl < S , Request > Service < Request > for ConcurrencyLimit < S > where S : Service < Request > , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . permit . is_none () { self . permit = ready ! (self . semaphore . poll_acquire (cx)) ; debug_assert ! (self . permit . is_some () , "ConcurrencyLimit semaphore is never closed, so `poll_acquire` \
                 should never fail" ,) ; } self . inner . poll_ready (cx) } fn call (& mut self , request : Request) -> Self :: Future { let permit = self . permit . take () . expect ("max requests in-flight; poll_ready must be called first") ; let future = self . inner . call (request) ; ResponseFuture :: new (future , permit) } }
};
}
