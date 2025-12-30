// Generated macro for ChaCha20Poly1305MessageEncrypter (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12ChaCha20Poly1305MessageEncrypter {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"ChaCha20Poly1305MessageEncrypter"}
// Dependencies: {}
# [doc = " The RFC7905/RFC7539 ChaCha20Poly1305 construction."] # [doc = " This implementation does the AAD construction required in TLS1.2."] # [doc = " TLS1.3 uses `TLS13MessageEncrypter`."] struct ChaCha20Poly1305MessageEncrypter { enc_key : aead :: LessSafeKey , enc_offset : Iv , }
};
}
