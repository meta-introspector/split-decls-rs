// Generated macro for impl_976 (impl)
macro_rules! Depcrate_crypto_ring_tls13impl_976 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"impl_976"}
// Dependencies: {}
impl Tls13AeadAlgorithm for Aes128GcmAead { fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > { self . 0 . encrypter (key , iv) } fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > { self . 0 . decrypter (key , iv) } fn key_len (& self) -> usize { self . 0 . key_len () } fn extract_keys (& self , key : AeadKey , iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Ok (ConnectionTrafficSecrets :: Aes128Gcm { key , iv }) } fn fips (& self) -> bool { super :: fips () } }
};
}
