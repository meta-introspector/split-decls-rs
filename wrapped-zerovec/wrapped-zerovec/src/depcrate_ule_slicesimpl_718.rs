// Generated macro for impl_718 (impl)
macro_rules! Depcrate_ule_slicesimpl_718 {
() => {
// Module: crate::ule::slices
// Provides: {"impl_718"}
// Dependencies: {}
unsafe impl < T : ULE , const N : usize > ULE for [T ; N] { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { T :: validate_bytes (bytes) } }
};
}
