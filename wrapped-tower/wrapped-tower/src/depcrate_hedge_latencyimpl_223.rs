// Generated macro for impl_223 (impl)
macro_rules! Depcrate_hedge_latencyimpl_223 {
() => {
// Module: crate::hedge::latency
// Provides: {"impl_223"}
// Dependencies: {}
impl < S , R , Request > Service < Request > for Latency < R , S > where S : Service < Request > , S :: Error : Into < crate :: BoxError > , R : Record + Clone , { type Response = S :: Response ; type Error = crate :: BoxError ; type Future = ResponseFuture < R , S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . service . poll_ready (cx) . map_err (Into :: into) } fn call (& mut self , request : Request) -> Self :: Future { ResponseFuture { start : Instant :: now () , rec : self . rec . clone () , inner : self . service . call (request) , } } }
};
}
