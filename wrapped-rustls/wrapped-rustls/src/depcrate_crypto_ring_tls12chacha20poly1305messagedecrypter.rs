// Generated macro for ChaCha20Poly1305MessageDecrypter (struct)
macro_rules! Depcrate_crypto_ring_tls12ChaCha20Poly1305MessageDecrypter {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"ChaCha20Poly1305MessageDecrypter"}
// Dependencies: {}
# [doc = " The RFC7905/RFC7539 ChaCha20Poly1305 construction."] # [doc = " This implementation does the AAD construction required in TLS1.2."] # [doc = " TLS1.3 uses `TLS13MessageDecrypter`."] struct ChaCha20Poly1305MessageDecrypter { dec_key : aead :: LessSafeKey , dec_offset : Iv , }
};
}
