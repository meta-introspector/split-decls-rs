// Generated macro for impl_461 (impl)
macro_rules! Depcrate_make_make_serviceimpl_461 {
() => {
// Module: crate::make::make_service
// Provides: {"impl_461"}
// Dependencies: {}
impl < M , S , Target , Request > Service < Target > for AsService < '_ , M , Request > where M : Service < Target , Response = S > , S : Service < Request > , { type Response = M :: Response ; type Error = M :: Error ; type Future = M :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . make . poll_ready (cx) } # [inline] fn call (& mut self , target : Target) -> Self :: Future { self . make . make_service (target) } }
};
}
