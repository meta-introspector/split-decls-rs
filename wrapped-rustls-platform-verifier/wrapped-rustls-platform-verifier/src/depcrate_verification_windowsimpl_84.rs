// Generated macro for impl_84 (impl)
macro_rules! Depcrate_verification_windowsimpl_84 {
() => {
// Module: crate::verification::windows
// Provides: {"impl_84"}
// Dependencies: {}
impl Drop for CertificateStore { fn drop (& mut self) { unsafe { CertCloseStore (self . inner . as_ptr () , 0) } ; } }
};
}
