// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl Tls13AeadAlgorithm for Aead { fn encrypter (& self , _key : AeadKey , _iv : Iv) -> Box < dyn MessageEncrypter > { Box :: new (Tls13Cipher) } fn decrypter (& self , _key : AeadKey , _iv : Iv) -> Box < dyn MessageDecrypter > { Box :: new (Tls13Cipher) } fn key_len (& self) -> usize { 32 } fn extract_keys (& self , _key : AeadKey , _iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Err (UnsupportedOperationError) } }
};
}
