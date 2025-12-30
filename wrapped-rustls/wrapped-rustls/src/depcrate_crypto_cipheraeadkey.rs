// Generated macro for AeadKey (struct)
macro_rules! Depcrate_crypto_cipherAeadKey {
() => {
// Module: crate::crypto::cipher
// Provides: {"AeadKey"}
// Dependencies: {}
# [doc = " A key for an AEAD algorithm."] # [doc = ""] # [doc = " This is a value type for a byte string up to `AeadKey::MAX_LEN` bytes in length."] pub struct AeadKey { buf : [u8 ; Self :: MAX_LEN] , used : usize , }
};
}
