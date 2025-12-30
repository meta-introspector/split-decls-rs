// Generated macro for AesKind (trait)
macro_rules! Depcrate_aes_ctrAesKind {
() => {
// Module: crate::aes_ctr
// Provides: {"AesKind"}
// Dependencies: {}
# [doc = " An AES cipher kind."] pub trait AesKind { # [doc = " Key type."] type Key : AsRef < [u8] > ; # [doc = " Cipher used to decrypt."] type Cipher : KeyInit ; }
};
}
