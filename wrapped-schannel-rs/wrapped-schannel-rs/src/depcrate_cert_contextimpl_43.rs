// Generated macro for impl_43 (impl)
macro_rules! Depcrate_cert_contextimpl_43 {
() => {
// Module: crate::cert_context
// Provides: {"impl_43"}
// Dependencies: {}
impl Drop for CertContext { fn drop (& mut self) { unsafe { Cryptography :: CertFreeCertificateContext (self . 0) ; } } }
};
}
