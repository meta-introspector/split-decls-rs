// Generated macro for impl_27 (impl)
macro_rules! Depcrate_versioned_sanitizedimpl_27 {
() => {
// Module: crate::versioned::sanitized
// Provides: {"impl_27"}
// Dependencies: {}
impl TryFrom < VersionedTransaction > for SanitizedVersionedTransaction { type Error = SanitizeError ; fn try_from (tx : VersionedTransaction) -> Result < Self , Self :: Error > { Self :: try_new (tx) } }
};
}
