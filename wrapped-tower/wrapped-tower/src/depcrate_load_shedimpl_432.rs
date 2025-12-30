// Generated macro for impl_432 (impl)
macro_rules! Depcrate_load_shedimpl_432 {
() => {
// Module: crate::load_shed
// Provides: {"impl_432"}
// Dependencies: {}
impl < S , Req > Service < Req > for LoadShed < S > where S : Service < Req > , S :: Error : Into < crate :: BoxError > , { type Response = S :: Response ; type Error = crate :: BoxError ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . is_ready = match self . inner . poll_ready (cx) { Poll :: Ready (Err (e)) => return Poll :: Ready (Err (e . into ())) , r => r . is_ready () , } ; Poll :: Ready (Ok (())) } fn call (& mut self , req : Req) -> Self :: Future { if self . is_ready { self . is_ready = false ; ResponseFuture :: called (self . inner . call (req)) } else { ResponseFuture :: overloaded () } } }
};
}
