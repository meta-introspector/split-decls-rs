// Generated macro for impl_116 (impl)
macro_rules! Depcrate_uleimpl_116 {
() => {
// Module: crate::ule
// Provides: {"impl_116"}
// Dependencies: {}
unsafe impl < const N : usize > ULE for UnvalidatedTinyAsciiStr < N > { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % N != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } Ok (()) } }
};
}
