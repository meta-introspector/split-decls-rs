// Generated macro for impl_343 (impl)
macro_rules! Depcrate_load_constantimpl_343 {
() => {
// Module: crate::load::constant
// Provides: {"impl_343"}
// Dependencies: {}
impl < S , M , Request > Service < Request > for Constant < S , M > where S : Service < Request > , M : Copy , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request) -> Self :: Future { self . inner . call (req) } }
};
}
