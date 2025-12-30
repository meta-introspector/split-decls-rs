// Generated macro for impl_331 (impl)
macro_rules! Depcrate_typesimpl_331 {
() => {
// Module: crate::types
// Provides: {"impl_331"}
// Dependencies: {}
# [cfg (feature = "aes-crypto")] impl AesMode { # [doc = " Length of the salt for the given AES mode."] pub const fn salt_length (& self) -> usize { self . key_length () / 2 } # [doc = " Length of the key for the given AES mode."] pub const fn key_length (& self) -> usize { match self { Self :: Aes128 => 16 , Self :: Aes192 => 24 , Self :: Aes256 => 32 , } } }
};
}
