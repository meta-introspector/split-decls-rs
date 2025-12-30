// Generated macro for impl_575 (impl)
macro_rules! Depcrate_ule_charsimpl_575 {
() => {
// Module: crate::ule::chars
// Provides: {"impl_575"}
// Dependencies: {}
unsafe impl ULE for CharULE { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . len () % 3 != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } for chunk in bytes . chunks_exact (3) { # [expect (clippy :: indexing_slicing)] let u = u32 :: from_le_bytes ([chunk [0] , chunk [1] , chunk [2] , 0]) ; char :: try_from (u) . map_err (| _ | UleError :: parse :: < Self > ()) ? ; } Ok (()) } }
};
}
