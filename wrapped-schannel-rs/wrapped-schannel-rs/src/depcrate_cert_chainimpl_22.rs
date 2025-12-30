// Generated macro for impl_22 (impl)
macro_rules! Depcrate_cert_chainimpl_22 {
() => {
// Module: crate::cert_chain
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'a > Iterator for CertificateChains < 'a > { type Item = CertChain ; fn next (& mut self) -> Option < CertChain > { let idx = self . idx ; self . idx += 1 ; self . context . get_chain (idx) } }
};
}
