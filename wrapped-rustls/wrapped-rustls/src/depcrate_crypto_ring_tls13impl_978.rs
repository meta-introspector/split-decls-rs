// Generated macro for impl_978 (impl)
macro_rules! Depcrate_crypto_ring_tls13impl_978 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"impl_978"}
// Dependencies: {}
impl AeadAlgorithm { fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > { Box :: new (Tls13MessageEncrypter { enc_key : aead :: LessSafeKey :: new (aead :: UnboundKey :: new (self . 0 , key . as_ref ()) . unwrap ()) , iv , }) } fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > { Box :: new (Tls13MessageDecrypter { dec_key : aead :: LessSafeKey :: new (aead :: UnboundKey :: new (self . 0 , key . as_ref ()) . unwrap ()) , iv , }) } fn key_len (& self) -> usize { self . 0 . key_len () } }
};
}
