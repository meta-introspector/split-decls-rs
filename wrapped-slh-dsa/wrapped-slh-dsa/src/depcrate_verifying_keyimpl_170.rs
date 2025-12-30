// Generated macro for impl_170 (impl)
macro_rules! Depcrate_verifying_keyimpl_170 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_170"}
// Dependencies: {}
impl < N : ArraySize > From < & [u8] > for PkSeed < N > { fn from (slice : & [u8]) -> Self { # [allow (deprecated)] Self (Array :: clone_from_slice (slice)) } }
};
}
