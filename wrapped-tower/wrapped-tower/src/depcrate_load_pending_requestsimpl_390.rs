// Generated macro for impl_390 (impl)
macro_rules! Depcrate_load_pending_requestsimpl_390 {
() => {
// Module: crate::load::pending_requests
// Provides: {"impl_390"}
// Dependencies: {}
impl < S , C , Request > Service < Request > for PendingRequests < S , C > where S : Service < Request > , C : TrackCompletion < Handle , S :: Response > , { type Response = C :: Output ; type Error = S :: Error ; type Future = TrackCompletionFuture < S :: Future , C , Handle > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . service . poll_ready (cx) } fn call (& mut self , req : Request) -> Self :: Future { TrackCompletionFuture :: new (self . completion . clone () , self . handle () , self . service . call (req) ,) } }
};
}
