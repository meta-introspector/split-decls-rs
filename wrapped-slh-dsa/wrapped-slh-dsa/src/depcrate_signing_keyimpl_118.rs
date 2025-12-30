// Generated macro for impl_118 (impl)
macro_rules! Depcrate_signing_keyimpl_118 {
() => {
// Module: crate::signing_key
// Provides: {"impl_118"}
// Dependencies: {}
impl < N : ArraySize > From < & [u8] > for SkSeed < N > { fn from (slice : & [u8]) -> Self { # [allow (deprecated)] Self (Array :: clone_from_slice (slice)) } }
};
}
