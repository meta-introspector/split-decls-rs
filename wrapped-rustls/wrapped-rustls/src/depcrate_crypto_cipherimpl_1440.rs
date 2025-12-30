// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1440 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1440"}
// Dependencies: {}
# [cfg (all (test , any (feature = "aws-lc-rs" , feature = "ring")))] impl Tls12AeadAlgorithm for FakeAead { fn encrypter (& self , _ : AeadKey , _ : & [u8] , _ : & [u8]) -> Box < dyn MessageEncrypter > { todo ! () } fn decrypter (& self , _ : AeadKey , _ : & [u8]) -> Box < dyn MessageDecrypter > { todo ! () } fn key_block_shape (& self) -> KeyBlockShape { todo ! () } fn extract_keys (& self , _ : AeadKey , _ : & [u8] , _ : & [u8] ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > { Err (UnsupportedOperationError) } fn fips (& self) -> bool { false } }
};
}
