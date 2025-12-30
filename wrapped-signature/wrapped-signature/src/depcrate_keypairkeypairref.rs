// Generated macro for KeypairRef (trait)
macro_rules! Depcrate_keypairKeypairRef {
() => {
// Module: crate::keypair
// Provides: {"KeypairRef"}
// Dependencies: {}
# [doc = " Signing keypair with an associated verifying key."] # [doc = ""] # [doc = " This represents a type which holds both a signing key and a verifying key."] pub trait KeypairRef : AsRef < Self :: VerifyingKey > { # [doc = " Verifying key type for this keypair."] type VerifyingKey : Clone ; }
};
}
