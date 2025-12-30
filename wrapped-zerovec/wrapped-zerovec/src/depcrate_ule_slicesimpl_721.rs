// Generated macro for impl_721 (impl)
macro_rules! Depcrate_ule_slicesimpl_721 {
() => {
// Module: crate::ule::slices
// Provides: {"impl_721"}
// Dependencies: {}
unsafe impl VarULE for str { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { core :: str :: from_utf8 (bytes) . map_err (| _ | UleError :: parse :: < Self > ()) ? ; Ok (()) } # [inline] fn parse_bytes (bytes : & [u8]) -> Result < & Self , UleError > { core :: str :: from_utf8 (bytes) . map_err (| _ | UleError :: parse :: < Self > ()) } # [doc = " Invariant: must be safe to call when called on a slice that previously"] # [doc = " succeeded with `parse_bytes`"] # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { core :: str :: from_utf8_unchecked (bytes) } }
};
}
