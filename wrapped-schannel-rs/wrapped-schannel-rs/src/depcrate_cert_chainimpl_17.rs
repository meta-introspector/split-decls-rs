// Generated macro for impl_17 (impl)
macro_rules! Depcrate_cert_chainimpl_17 {
() => {
// Module: crate::cert_chain
// Provides: {"impl_17"}
// Dependencies: {}
impl Drop for CertChainContext { fn drop (& mut self) { unsafe { Cryptography :: CertFreeCertificateChain (self . 0) ; } } }
};
}
