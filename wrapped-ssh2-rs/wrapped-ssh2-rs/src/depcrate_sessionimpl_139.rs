// Generated macro for impl_139 (impl)
macro_rules! Depcrate_sessionimpl_139 {
() => {
// Module: crate::session
// Provides: {"impl_139"}
// Dependencies: {}
impl Drop for SessionInner { fn drop (& mut self) { unsafe { let _rc = raw :: libssh2_session_free (self . raw) ; } } }
};
}
