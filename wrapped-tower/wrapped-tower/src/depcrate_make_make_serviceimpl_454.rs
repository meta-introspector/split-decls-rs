// Generated macro for impl_454 (impl)
macro_rules! Depcrate_make_make_serviceimpl_454 {
() => {
// Module: crate::make::make_service
// Provides: {"impl_454"}
// Dependencies: {}
impl < M , S , Target , Request > MakeService < Target , Request > for M where M : Service < Target , Response = S > , S : Service < Request > , { type Response = S :: Response ; type Error = S :: Error ; type Service = S ; type MakeError = M :: Error ; type Future = M :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: MakeError > > { Service :: poll_ready (self , cx) } fn make_service (& mut self , target : Target) -> Self :: Future { Service :: call (self , target) } }
};
}
