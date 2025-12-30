// Generated macro for impl_80 (impl)
macro_rules! Depcrate_verification_windowsimpl_80 {
() => {
// Module: crate::verification::windows
// Provides: {"impl_80"}
// Dependencies: {}
impl Drop for CertEngine { fn drop (& mut self) { unsafe { CertFreeCertificateChainEngine (EnginePtr :: from_raw (self . inner)) } ; } }
};
}
