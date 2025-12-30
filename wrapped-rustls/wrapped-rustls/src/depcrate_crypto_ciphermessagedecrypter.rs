// Generated macro for MessageDecrypter (trait)
macro_rules! Depcrate_crypto_cipherMessageDecrypter {
() => {
// Module: crate::crypto::cipher
// Provides: {"MessageDecrypter"}
// Dependencies: {}
# [doc = " Objects with this trait can decrypt TLS messages."] pub trait MessageDecrypter : Send + Sync { # [doc = " Decrypt the given TLS message `msg`, using the sequence number"] # [doc = " `seq` which can be used to derive a unique [`Nonce`]."] fn decrypt < 'a > (& mut self , msg : InboundOpaqueMessage < 'a > , seq : u64 ,) -> Result < InboundPlainMessage < 'a > , Error > ; }
};
}
