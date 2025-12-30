// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls13impl_1316 {
() => {
// Module: crate::crypto::aws_lc_rs::tls13
// Provides: {"impl_1316"}
// Dependencies: {}
impl AeadAlgorithm { fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > { Box :: new (GcmMessageEncrypter { enc_key : aead :: TlsRecordSealingKey :: new (self . 0 , aead :: TlsProtocolId :: TLS13 , key . as_ref () ,) . unwrap () , iv , }) } fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > { Box :: new (GcmMessageDecrypter { dec_key : aead :: TlsRecordOpeningKey :: new (self . 0 , aead :: TlsProtocolId :: TLS13 , key . as_ref () ,) . unwrap () , iv , }) } fn key_len (& self) -> usize { self . 0 . key_len () } }
};
}
