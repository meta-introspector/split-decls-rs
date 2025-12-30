// Generated macro for SignatureLen (trait)
macro_rules! Depcrate_signature_encodingSignatureLen {
() => {
// Module: crate::signature_encoding
// Provides: {"SignatureLen"}
// Dependencies: {}
# [doc = " A trait specifying the length of a serialized signature for a given parameter set"] pub trait SignatureLen { # [doc = " The length of the signature in bytes"] type SigLen : ArraySize ; }
};
}
