// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_ticketerimpl_1244 {
() => {
// Module: crate::crypto::aws_lc_rs::ticketer
// Provides: {"impl_1244"}
// Dependencies: {}
impl Rfc5077Ticketer { # [expect (clippy :: new_ret_no_self)] pub (super) fn new () -> Result < Box < dyn TicketProducer > , Error > { let rand = SystemRandom :: new () ; let mut aes_key = [0u8 ; AES_256_KEY_LEN] ; rand . fill (& mut aes_key) . map_err (| _ | GetRandomFailed) ? ; let aes_encrypt_key = UnboundCipherKey :: new (& AES_256 , & aes_key [..]) . map_err (unspecified_err) ? ; let aes_encrypt_key = PaddedBlockEncryptingKey :: cbc_pkcs7 (aes_encrypt_key) . map_err (unspecified_err) ? ; let aes_decrypt_key = UnboundCipherKey :: new (& AES_256 , & aes_key [..]) . map_err (unspecified_err) ? ; let aes_decrypt_key = PaddedBlockDecryptingKey :: cbc_pkcs7 (aes_decrypt_key) . map_err (unspecified_err) ? ; let hmac_key = hmac :: Key :: generate (hmac :: HMAC_SHA256 , & rand) . map_err (unspecified_err) ? ; let mut key_name = [0u8 ; 16] ; rand . fill (& mut key_name) . map_err (| _ | GetRandomFailed) ? ; Ok (Box :: new (Self { aes_encrypt_key , aes_decrypt_key , hmac_key , key_name , maximum_ciphertext_len : AtomicUsize :: new (0) , })) } }
};
}
