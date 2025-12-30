// Generated macro for impl_1436 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1436 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1436"}
// Dependencies: {}
impl MessageEncrypter for InvalidMessageEncrypter { fn encrypt (& mut self , _m : OutboundPlainMessage < '_ > , _seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > { Err (Error :: EncryptError) } fn encrypted_payload_len (& self , payload_len : usize) -> usize { payload_len } }
};
}
