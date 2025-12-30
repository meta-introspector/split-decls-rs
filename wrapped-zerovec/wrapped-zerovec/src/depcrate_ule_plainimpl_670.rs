// Generated macro for impl_670 (impl)
macro_rules! Depcrate_ule_plainimpl_670 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_670"}
// Dependencies: {}
unsafe impl < const N : usize > ULE for RawBytesULE < N > { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % N == 0 { Ok (()) } else { Err (UleError :: length :: < Self > (bytes . len ())) } } }
};
}
