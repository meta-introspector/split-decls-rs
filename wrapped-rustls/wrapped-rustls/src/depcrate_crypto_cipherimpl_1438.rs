// Generated macro for impl_1438 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1438 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1438"}
// Dependencies: {}
impl MessageDecrypter for InvalidMessageDecrypter { fn decrypt < 'a > (& mut self , _m : InboundOpaqueMessage < 'a > , _seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > { Err (Error :: DecryptError) } }
};
}
