// Generated macro for impl_972 (impl)
macro_rules! Depcrate_crypto_ring_tls13impl_972 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"impl_972"}
// Dependencies: {}
impl Tls13AeadAlgorithm for Chacha20Poly1305Aead { fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > { self . 0 . encrypter (key , iv) } fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > { self . 0 . decrypter (key , iv) } fn key_len (& self) -> usize { self . 0 . key_len () } fn extract_keys (& self , key : AeadKey , iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Ok (ConnectionTrafficSecrets :: Chacha20Poly1305 { key , iv }) } fn fips (& self) -> bool { false } }
};
}
