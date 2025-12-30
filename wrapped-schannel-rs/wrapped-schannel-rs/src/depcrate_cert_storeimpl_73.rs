// Generated macro for impl_73 (impl)
macro_rules! Depcrate_cert_storeimpl_73 {
() => {
// Module: crate::cert_store
// Provides: {"impl_73"}
// Dependencies: {}
impl Drop for CertStore { fn drop (& mut self) { unsafe { Cryptography :: CertCloseStore (self . 0 , 0) ; } } }
};
}
