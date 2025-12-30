// Generated macro for impl_228 (impl)
macro_rules! Depcrate_security_contextimpl_228 {
() => {
// Module: crate::security_context
// Provides: {"impl_228"}
// Dependencies: {}
impl Inner < Credentials :: SecHandle > for SecurityContext { unsafe fn from_inner (inner : Credentials :: SecHandle) -> SecurityContext { SecurityContext (inner) } fn as_inner (& self) -> Credentials :: SecHandle { self . 0 } fn get_mut (& mut self) -> & mut Credentials :: SecHandle { & mut self . 0 } }
};
}
