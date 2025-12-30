// Generated macro for xchacha20_poly1305 (function)
macro_rules! Depcrate_aeadxchacha20_poly1305 {
() => {
// Module: crate::aead
// Provides: {"xchacha20_poly1305"}
// Dependencies: {}
pub fn xchacha20_poly1305 (data : & [u8] , algorithm : & str , _key_size : u32) -> Vec < TestInfo > { generator (data , algorithm , 256 , 24 * 8) }
};
}
