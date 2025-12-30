// Generated macro for HpkeOpener (trait)
macro_rules! Depcrate_crypto_hpkeHpkeOpener {
() => {
// Module: crate::crypto::hpke
// Provides: {"HpkeOpener"}
// Dependencies: {}
# [doc = " An HPKE opener context."] # [doc = ""] # [doc = " This is a stateful object that can be used to open sealed messages sealed"] # [doc = " by a sender."] pub trait HpkeOpener : Debug + Send + Sync + 'static { # [doc = " Open the provided `ciphertext` with additional data `aad`, returning plaintext."] fn open (& mut self , aad : & [u8] , ciphertext : & [u8]) -> Result < Vec < u8 > , Error > ; }
};
}
