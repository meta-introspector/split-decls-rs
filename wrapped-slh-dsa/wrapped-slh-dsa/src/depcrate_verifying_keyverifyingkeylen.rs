// Generated macro for VerifyingKeyLen (trait)
macro_rules! Depcrate_verifying_keyVerifyingKeyLen {
() => {
// Module: crate::verifying_key
// Provides: {"VerifyingKeyLen"}
// Dependencies: {}
# [doc = " A trait specifying the length of a serialized verifying key for a given parameter set"] pub trait VerifyingKeyLen { # [doc = " The length of the serialized verifying key in bytes"] type VkLen : ArraySize ; }
};
}
