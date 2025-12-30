// Generated macro for impl_122 (impl)
macro_rules! Depcrate_signing_keyimpl_122 {
() => {
// Module: crate::signing_key
// Provides: {"impl_122"}
// Dependencies: {}
impl < N : ArraySize > From < & [u8] > for SkPrf < N > { fn from (slice : & [u8]) -> Self { # [allow (deprecated)] Self (Array :: clone_from_slice (slice)) } }
};
}
