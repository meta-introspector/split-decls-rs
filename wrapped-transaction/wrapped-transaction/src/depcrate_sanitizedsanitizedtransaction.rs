// Generated macro for SanitizedTransaction (struct)
macro_rules! Depcrate_sanitizedSanitizedTransaction {
() => {
// Module: crate::sanitized
// Provides: {"SanitizedTransaction"}
// Dependencies: {}
# [doc = " Sanitized transaction and the hash of its message"] # [derive (Debug , Clone , Eq , PartialEq)] pub struct SanitizedTransaction { message : SanitizedMessage , message_hash : Hash , is_simple_vote_tx : bool , signatures : Vec < Signature > , }
};
}
