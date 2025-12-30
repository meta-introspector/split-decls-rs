// Generated macro for HpkeKeyPair (struct)
macro_rules! Depcrate_crypto_hpkeHpkeKeyPair {
() => {
// Module: crate::crypto::hpke
// Provides: {"HpkeKeyPair"}
// Dependencies: {}
# [doc = " An HPKE key pair, made of a matching public and private key."] # [expect (clippy :: exhaustive_structs)] pub struct HpkeKeyPair { # [doc = " A HPKE public key."] pub public_key : HpkePublicKey , # [doc = " A HPKE private key."] pub private_key : HpkePrivateKey , }
};
}
