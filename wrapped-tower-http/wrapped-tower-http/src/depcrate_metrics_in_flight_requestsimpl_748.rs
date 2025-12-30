// Generated macro for impl_748 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_748 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_748"}
// Dependencies: {}
impl < S , R , ResBody > Service < Request < R > > for InFlightRequests < S > where S : Service < Request < R > , Response = Response < ResBody > > , { type Response = Response < ResponseBody < ResBody > > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < R >) -> Self :: Future { let guard = self . counter . increment () ; ResponseFuture { inner : self . inner . call (req) , guard : Some (guard) , } } }
};
}
