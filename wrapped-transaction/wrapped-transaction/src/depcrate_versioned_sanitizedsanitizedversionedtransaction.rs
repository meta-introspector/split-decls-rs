// Generated macro for SanitizedVersionedTransaction (struct)
macro_rules! Depcrate_versioned_sanitizedSanitizedVersionedTransaction {
() => {
// Module: crate::versioned::sanitized
// Provides: {"SanitizedVersionedTransaction"}
// Dependencies: {}
# [doc = " Wraps a sanitized `VersionedTransaction` to provide a safe API"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SanitizedVersionedTransaction { # [doc = " List of signatures"] pub (crate) signatures : Vec < Signature > , # [doc = " Message to sign."] pub (crate) message : SanitizedVersionedMessage , }
};
}
