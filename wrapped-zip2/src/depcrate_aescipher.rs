// Generated macro for Cipher (enum)
macro_rules! Depcrate_aesCipher {
() => {
// Module: crate::aes
// Provides: {"Cipher"}
// Dependencies: {}
enum Cipher { Aes128 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes128 > >) , Aes192 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes192 > >) , Aes256 (Box < aes_ctr :: AesCtrZipKeyStream < aes_ctr :: Aes256 > >) , }
};
}
