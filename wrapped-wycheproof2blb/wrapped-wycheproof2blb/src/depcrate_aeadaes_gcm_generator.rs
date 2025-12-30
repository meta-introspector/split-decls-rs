// Generated macro for aes_gcm_generator (function)
macro_rules! Depcrate_aeadaes_gcm_generator {
() => {
// Module: crate::aead
// Provides: {"aes_gcm_generator"}
// Dependencies: {}
pub fn aes_gcm_generator (data : & [u8] , algorithm : & str , key_size : u32) -> Vec < TestInfo > { generator (data , algorithm , key_size , 12 * 8) }
};
}
