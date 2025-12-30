// Generated macro for SigningKeyLen (trait)
macro_rules! Depcrate_signing_keySigningKeyLen {
() => {
// Module: crate::signing_key
// Provides: {"SigningKeyLen"}
// Dependencies: {}
# [doc = " A trait specifying the length of a serialized signing key for a given parameter set"] pub trait SigningKeyLen : VerifyingKeyLen { # [doc = " The length of the serialized signing key in bytes"] type SkLen : ArraySize ; }
};
}
