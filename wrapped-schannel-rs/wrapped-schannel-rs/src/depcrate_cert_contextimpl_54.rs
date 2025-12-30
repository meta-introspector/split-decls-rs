// Generated macro for impl_54 (impl)
macro_rules! Depcrate_cert_contextimpl_54 {
() => {
// Module: crate::cert_context
// Provides: {"impl_54"}
// Dependencies: {}
impl KeySpec { # [doc = " A key used to encrypt/decrypt session keys."] pub fn key_exchange () -> KeySpec { KeySpec (Cryptography :: AT_KEYEXCHANGE) } # [doc = " A key used to create and verify digital signatures."] pub fn signature () -> KeySpec { KeySpec (Cryptography :: AT_SIGNATURE) } }
};
}
