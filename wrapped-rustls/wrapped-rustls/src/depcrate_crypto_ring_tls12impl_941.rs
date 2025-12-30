// Generated macro for impl_941 (impl)
macro_rules! Depcrate_crypto_ring_tls12impl_941 {
() => {
// Module: crate::crypto::ring::tls12
// Provides: {"impl_941"}
// Dependencies: {}
impl Tls12AeadAlgorithm for GcmAlgorithm { fn decrypter (& self , dec_key : AeadKey , dec_iv : & [u8]) -> Box < dyn MessageDecrypter > { let dec_key = aead :: LessSafeKey :: new (aead :: UnboundKey :: new (self . 0 , dec_key . as_ref ()) . unwrap ()) ; let mut ret = GcmMessageDecrypter { dec_key , dec_salt : [0u8 ; 4] , } ; debug_assert_eq ! (dec_iv . len () , 4) ; ret . dec_salt . copy_from_slice (dec_iv) ; Box :: new (ret) } fn encrypter (& self , enc_key : AeadKey , write_iv : & [u8] , explicit : & [u8] ,) -> Box < dyn MessageEncrypter > { let enc_key = aead :: LessSafeKey :: new (aead :: UnboundKey :: new (self . 0 , enc_key . as_ref ()) . unwrap ()) ; let iv = gcm_iv (write_iv , explicit) ; Box :: new (GcmMessageEncrypter { enc_key , iv }) } fn key_block_shape (& self) -> KeyBlockShape { KeyBlockShape { enc_key_len : self . 0 . key_len () , fixed_iv_len : 4 , explicit_nonce_len : 8 , } } fn extract_keys (& self , key : AeadKey , write_iv : & [u8] , explicit : & [u8] ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { let iv = gcm_iv (write_iv , explicit) ; Ok (match self . 0 . key_len () { 16 => ConnectionTrafficSecrets :: Aes128Gcm { key , iv } , 32 => ConnectionTrafficSecrets :: Aes256Gcm { key , iv } , _ => unreachable ! () , }) } fn fips (& self) -> bool { super :: fips () } }
};
}
