// Generated macro for impl_74 (impl)
macro_rules! Depcrate_verification_windowsimpl_74 {
() => {
// Module: crate::verification::windows
// Provides: {"impl_74"}
// Dependencies: {}
impl Drop for CertChain { fn drop (& mut self) { unsafe { CertFreeCertificateChain (self . inner . as_ptr ()) } } }
};
}
