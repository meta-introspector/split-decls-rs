// Generated macro for impl_943 (impl)
macro_rules! Depcrate_crypto_ring_tls12impl_943 {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"impl_943"}
// Dependencies: {}
impl Tls12AeadAlgorithm for ChaCha20Poly1305 { fn decrypter (& self , dec_key : AeadKey , iv : & [u8]) -> Box < dyn MessageDecrypter > { let dec_key = aead :: LessSafeKey :: new (aead :: UnboundKey :: new (& aead :: CHACHA20_POLY1305 , dec_key . as_ref ()) . unwrap () ,) ; Box :: new (ChaCha20Poly1305MessageDecrypter { dec_key , dec_offset : Iv :: new (iv) . expect ("IV length validated by key_block_shape") , }) } fn encrypter (& self , enc_key : AeadKey , enc_iv : & [u8] , _ : & [u8]) -> Box < dyn MessageEncrypter > { let enc_key = aead :: LessSafeKey :: new (aead :: UnboundKey :: new (& aead :: CHACHA20_POLY1305 , enc_key . as_ref ()) . unwrap () ,) ; Box :: new (ChaCha20Poly1305MessageEncrypter { enc_key , enc_offset : Iv :: new (enc_iv) . expect ("IV length validated by key_block_shape") , }) } fn key_block_shape (& self) -> KeyBlockShape { KeyBlockShape { enc_key_len : 32 , fixed_iv_len : 12 , explicit_nonce_len : 0 , } } fn extract_keys (& self , key : AeadKey , iv : & [u8] , _explicit : & [u8] ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { debug_assert_eq ! (aead :: NONCE_LEN , iv . len ()) ; Ok (ConnectionTrafficSecrets :: Chacha20Poly1305 { key , iv : Iv :: new (iv) . expect ("IV length validated by key_block_shape") , }) } fn fips (& self) -> bool { false } }
};
}
