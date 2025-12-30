// Generated macro for impl_606 (impl)
macro_rules! Depcrate_spawn_ready_serviceimpl_606 {
() => {
// Module: crate::spawn_ready::service
// Provides: {"impl_606"}
// Dependencies: {}
impl < S , Req > Service < Req > for SpawnReady < S > where Req : 'static , S : Service < Req > + Send + 'static , S :: Error : Into < BoxError > , { type Response = S :: Response ; type Error = BoxError ; type Future = ResponseFuture < S :: Future , S :: Error > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , BoxError > > { loop { self . inner = match self . inner { Inner :: Service (ref mut svc) => { if let Poll :: Ready (r) = svc . as_mut () . expect ("illegal state") . poll_ready (cx) { return Poll :: Ready (r . map_err (Into :: into)) ; } let svc = svc . take () . expect ("illegal state") ; let rx = tokio :: spawn (svc . ready_oneshot () . map_err (Into :: into) . in_current_span ()) ; Inner :: Future (rx) } Inner :: Future (ref mut fut) => { let svc = ready ! (Pin :: new (fut) . poll (cx)) ? ? ; Inner :: Service (Some (svc)) } } } } fn call (& mut self , request : Req) -> Self :: Future { match self . inner { Inner :: Service (Some (ref mut svc)) => { ResponseFuture :: new (svc . call (request) . map_err (Into :: into)) } _ => unreachable ! ("poll_ready must be called") , } } }
};
}
