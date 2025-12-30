// Generated macro for impl_77 (impl)
macro_rules! Depcrate_verification_windowsimpl_77 {
() => {
// Module: crate::verification::windows
// Provides: {"impl_77"}
// Dependencies: {}
impl Drop for Certificate { fn drop (& mut self) { unsafe { CertFreeCertificateContext (self . inner . as_ptr ()) } ; } }
};
}
