// Generated macro for MessageEncrypter (trait)
macro_rules! Depcrate_crypto_cipherMessageEncrypter {
() => {
// Module: crate::crypto::cipher
// Provides: {"MessageEncrypter"}
// Dependencies: {}
# [doc = " Objects with this trait can encrypt TLS messages."] pub trait MessageEncrypter : Send + Sync { # [doc = " Encrypt the given TLS message `msg`, using the sequence number"] # [doc = " `seq` which can be used to derive a unique [`Nonce`]."] fn encrypt (& mut self , msg : OutboundPlainMessage < '_ > , seq : u64 ,) -> Result < OutboundOpaqueMessage , Error > ; # [doc = " Return the length of the ciphertext that results from encrypting plaintext of"] # [doc = " length `payload_len`"] fn encrypted_payload_len (& self , payload_len : usize) -> usize ; }
};
}
