// Generated macro for impl_1414 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1414 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1414"}
// Dependencies: {}
impl From < [u8 ; NONCE_LEN] > for Iv { fn from (bytes : [u8 ; NONCE_LEN]) -> Self { Self :: new (& bytes) . expect ("NONCE_LEN is within MAX_LEN") } }
};
}
