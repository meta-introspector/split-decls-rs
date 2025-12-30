// Generated macro for impl_24 (impl)
macro_rules! Depcrate_cert_chainimpl_24 {
() => {
// Module: crate::cert_chain
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > Iterator for Certificates < 'a > { type Item = CertContext ; fn next (& mut self) -> Option < CertContext > { let idx = self . idx ; self . idx += 1 ; self . chain . get (idx) } }
};
}
