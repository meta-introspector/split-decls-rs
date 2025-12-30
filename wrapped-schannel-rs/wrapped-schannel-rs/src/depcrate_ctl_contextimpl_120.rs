// Generated macro for impl_120 (impl)
macro_rules! Depcrate_ctl_contextimpl_120 {
() => {
// Module: crate::ctl_context
// Provides: {"impl_120"}
// Dependencies: {}
impl Drop for CtlContext { fn drop (& mut self) { unsafe { Cryptography :: CertFreeCTLContext (self . 0) ; } } }
};
}
