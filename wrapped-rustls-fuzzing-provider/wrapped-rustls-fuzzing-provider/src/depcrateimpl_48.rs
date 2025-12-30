// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl Tls12AeadAlgorithm for Aead { fn encrypter (& self , _key : AeadKey , _iv : & [u8] , _ : & [u8]) -> Box < dyn MessageEncrypter > { Box :: new (Tls12Cipher) } fn decrypter (& self , _key : AeadKey , _iv : & [u8]) -> Box < dyn MessageDecrypter > { Box :: new (Tls12Cipher) } fn key_block_shape (& self) -> KeyBlockShape { KeyBlockShape { enc_key_len : 32 , fixed_iv_len : 12 , explicit_nonce_len : 0 , } } fn extract_keys (& self , _key : AeadKey , _iv : & [u8] , _explicit : & [u8] ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Err (UnsupportedOperationError) } }
};
}
