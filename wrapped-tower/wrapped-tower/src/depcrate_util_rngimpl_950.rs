// Generated macro for impl_950 (impl)
macro_rules! Depcrate_util_rngimpl_950 {
() => {
// Module: crate::util::rng
// Provides: {"impl_950"}
// Dependencies: {}
impl < H > HasherRng < H > { # [doc = " Create a new [`HasherRng`] with the provided hasher."] pub fn with_hasher (hasher : H) -> Self { HasherRng { hasher , counter : 0 } } }
};
}
