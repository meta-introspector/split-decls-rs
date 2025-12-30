// Generated macro for impl_432 (impl)
macro_rules! Depcrate_quic_addr_validation_tokenimpl_432 {
() => {
// Module: crate::quic::addr_validation_token
// Provides: {"impl_432"}
// Dependencies: {}
impl Default for AddrValidationTokenManager { fn default () -> Self { let mut key_bytes = [0 ; HMAC_KEY_LEN] ; boring :: rand :: rand_bytes (& mut key_bytes) . unwrap () ; AddrValidationTokenManager { sign_key : key_bytes , } } }
};
}
