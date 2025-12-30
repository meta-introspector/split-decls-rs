// Generated macro for impl_443 (impl)
macro_rules! Depcrate_make_make_connectionimpl_443 {
() => {
// Module: crate::make::make_connection
// Provides: {"impl_443"}
// Dependencies: {}
impl < C , Target > MakeConnection < Target > for C where C : Service < Target > , C :: Response : AsyncRead + AsyncWrite , { type Connection = C :: Response ; type Error = C :: Error ; type Future = C :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Service :: poll_ready (self , cx) } fn make_connection (& mut self , target : Target) -> Self :: Future { Service :: call (self , target) } }
};
}
