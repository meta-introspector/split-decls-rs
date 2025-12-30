// Generated macro for chacha20_poly1305 (function)
macro_rules! Depcrate_aeadchacha20_poly1305 {
() => {
// Module: crate::aead
// Provides: {"chacha20_poly1305"}
// Dependencies: {}
pub fn chacha20_poly1305 (data : & [u8] , algorithm : & str , _key_size : u32) -> Vec < TestInfo > { generator (data , algorithm , 256 , 12 * 8) }
};
}
