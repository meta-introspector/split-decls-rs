// Generated macro for impl_713 (impl)
macro_rules! Depcrate_ule_plainimpl_713 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_713"}
// Dependencies: {}
unsafe impl ULE for () { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { if bytes . is_empty () { Ok (()) } else { Err (UleError :: length :: < Self > (bytes . len ())) } } }
};
}
