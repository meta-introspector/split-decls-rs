// Generated macro for impl_100 (impl)
macro_rules! Depcrate_crypt_provimpl_100 {
() => {
// Module: crate::crypt_prov
// Provides: {"impl_100"}
// Dependencies: {}
impl Drop for CryptProv { fn drop (& mut self) { unsafe { Cryptography :: CryptReleaseContext (self . 0 , 0) ; } } }
};
}
