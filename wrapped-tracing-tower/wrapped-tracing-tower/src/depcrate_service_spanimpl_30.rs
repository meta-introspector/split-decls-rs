// Generated macro for impl_30 (impl)
macro_rules! Depcrate_service_spanimpl_30 {
() => {
// Module: crate::service_span
// Provides: {"impl_30"}
// Dependencies: {}
impl < S , R > tower_service :: Service < R > for Service < S > where S : tower_service :: Service < R > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let _enter = self . span . enter () ; self . inner . poll_ready (cx) } fn call (& mut self , request : R) -> Self :: Future { let _enter = self . span . enter () ; self . inner . call (request) } }
};
}
