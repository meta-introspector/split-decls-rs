// Generated macro for AesCipher (trait)
macro_rules! Depcrate_aes_ctrAesCipher {
() => {
// Module: crate::aes_ctr
// Provides: {"AesCipher"}
// Dependencies: {}
# [doc = " This trait allows using generic AES ciphers with different key sizes."] pub trait AesCipher { fn crypt_in_place (& mut self , target : & mut [u8]) ; }
};
}
