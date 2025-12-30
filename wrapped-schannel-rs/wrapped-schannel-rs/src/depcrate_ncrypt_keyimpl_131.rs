// Generated macro for impl_131 (impl)
macro_rules! Depcrate_ncrypt_keyimpl_131 {
() => {
// Module: crate::ncrypt_key
// Provides: {"impl_131"}
// Dependencies: {}
impl Drop for NcryptKey { fn drop (& mut self) { unsafe { Cryptography :: NCryptFreeObject (self . 0) ; } } }
};
}
