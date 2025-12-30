// Generated macro for impl_1310 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls13impl_1310 {
() => {
// Module: crate::crypto::aws_lc_rs::tls13
// Provides: {"impl_1310"}
// Dependencies: {}
impl Tls13AeadAlgorithm for Chacha20Poly1305Aead { fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > { Box :: new (AeadMessageEncrypter { enc_key : aead :: LessSafeKey :: new (aead :: UnboundKey :: new (self . 0 . 0 , key . as_ref ()) . unwrap ()) , iv , }) } fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > { Box :: new (AeadMessageDecrypter { dec_key : aead :: LessSafeKey :: new (aead :: UnboundKey :: new (self . 0 . 0 , key . as_ref ()) . unwrap ()) , iv , }) } fn key_len (& self) -> usize { self . 0 . key_len () } fn extract_keys (& self , key : AeadKey , iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Ok (ConnectionTrafficSecrets :: Chacha20Poly1305 { key , iv }) } fn fips (& self) -> bool { false } }
};
}
