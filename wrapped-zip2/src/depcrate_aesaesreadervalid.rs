// Generated macro for AesReaderValid (struct)
macro_rules! Depcrate_aesAesReaderValid {
() => {
// Module: crate::aes
// Provides: {"AesReaderValid"}
// Dependencies: {}
# [doc = " A reader for aes encrypted files, which has already passed the first password check."] # [doc = ""] # [doc = " There is a 1 in 65536 chance that an invalid password passes that check."] # [doc = " After the data has been read and decrypted an HMAC will be checked and provide a final means"] # [doc = " to check if either the password is invalid or if the data has been changed."] pub struct AesReaderValid < R : Read > { reader : R , data_remaining : u64 , cipher : Cipher , hmac : Hmac < Sha1 > , finalized : bool , }
};
}
