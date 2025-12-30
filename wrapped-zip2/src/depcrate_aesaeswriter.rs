// Generated macro for AesWriter (struct)
macro_rules! Depcrate_aesAesWriter {
() => {
// Module: crate::aes
// Provides: {"AesWriter"}
// Dependencies: {}
pub struct AesWriter < W > { writer : W , cipher : Cipher , hmac : Hmac < Sha1 > , buffer : Zeroizing < Vec < u8 > > , encrypted_file_header : Option < Vec < u8 > > , }
};
}
