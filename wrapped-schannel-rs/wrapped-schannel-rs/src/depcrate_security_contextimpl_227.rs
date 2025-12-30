// Generated macro for impl_227 (impl)
macro_rules! Depcrate_security_contextimpl_227 {
() => {
// Module: crate::security_context
// Provides: {"impl_227"}
// Dependencies: {}
impl Drop for SecurityContext { fn drop (& mut self) { unsafe { Identity :: DeleteSecurityContext (& self . 0) ; } } }
};
}
