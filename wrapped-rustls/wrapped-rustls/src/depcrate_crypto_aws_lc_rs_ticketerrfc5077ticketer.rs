// Generated macro for Rfc5077Ticketer (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_ticketerRfc5077Ticketer {
() => {
// Module: crate::crypto::aws_lc_rs::ticketer
// Provides: {"Rfc5077Ticketer"}
// Dependencies: {}
# [doc = " An RFC 5077 \"Recommended Ticket Construction\" implementation of a [`TicketProducer`]."] pub (super) struct Rfc5077Ticketer { aes_encrypt_key : PaddedBlockEncryptingKey , aes_decrypt_key : PaddedBlockDecryptingKey , hmac_key : hmac :: Key , key_name : [u8 ; 16] , maximum_ciphertext_len : AtomicUsize , }
};
}
