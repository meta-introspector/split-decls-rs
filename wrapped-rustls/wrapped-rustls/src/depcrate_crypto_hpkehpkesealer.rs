// Generated macro for HpkeSealer (trait)
macro_rules! Depcrate_crypto_hpkeHpkeSealer {
() => {
// Module: crate::crypto::hpke
// Provides: {"HpkeSealer"}
// Dependencies: {}
# [doc = " An HPKE sealer context."] # [doc = ""] # [doc = " This is a stateful object that can be used to seal messages for receipt by"] # [doc = " a receiver."] pub trait HpkeSealer : Debug + Send + Sync + 'static { # [doc = " Seal the provided `plaintext` with additional data `aad`, returning"] # [doc = " ciphertext."] fn seal (& mut self , aad : & [u8] , plaintext : & [u8]) -> Result < Vec < u8 > , Error > ; }
};
}
