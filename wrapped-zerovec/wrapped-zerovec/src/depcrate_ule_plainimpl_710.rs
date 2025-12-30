// Generated macro for impl_710 (impl)
macro_rules! Depcrate_ule_plainimpl_710 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_710"}
// Dependencies: {}
unsafe impl ULE for bool { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { for byte in bytes { if * byte > 1 { return Err (UleError :: parse :: < Self > ()) ; } } Ok (()) } }
};
}
