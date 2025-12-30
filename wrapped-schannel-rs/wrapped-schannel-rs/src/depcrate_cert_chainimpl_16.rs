// Generated macro for impl_16 (impl)
macro_rules! Depcrate_cert_chainimpl_16 {
() => {
// Module: crate::cert_chain
// Provides: {"impl_16"}
// Dependencies: {}
impl Clone for CertChainContext { fn clone (& self) -> Self { let rced = unsafe { Cryptography :: CertDuplicateCertificateChain (self . 0) } ; CertChainContext (rced) } }
};
}
