// Generated macro for impl_16 (impl)
macro_rules! Depcrate_request_spanimpl_16 {
() => {
// Module: crate::request_span
// Provides: {"impl_16"}
// Dependencies: {}
impl < S , R , G > tower_service :: Service < R > for Service < S , R , G > where S : tower_service :: Service < R > , G : GetSpan < R > + Clone , { type Response = S :: Response ; type Error = S :: Error ; type Future = tracing :: instrument :: Instrumented < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , request : R) -> Self :: Future { let span = self . get_span . span_for (& request) ; let _enter = span . enter () ; self . inner . call (request) . instrument (span . clone ()) } }
};
}
