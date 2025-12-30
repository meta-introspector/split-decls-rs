// Generated macro for impl_648 (impl)
macro_rules! Depcrate_ule_optionimpl_648 {
() => {
// Module: crate::ule::option
// Provides: {"impl_648"}
// Dependencies: {}
# [doc = "    zeroed or valid-T byte sequences to fill it)"] unsafe impl < U : ULE > ULE for OptionULE < U > { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { let size = mem :: size_of :: < Self > () ; if bytes . len () % size != 0 { return Err (UleError :: length :: < Self > (bytes . len ())) ; } for chunk in bytes . chunks (size) { # [expect (clippy :: indexing_slicing)] match chunk [0] { 0 => { if ! chunk [1 ..] . iter () . all (| x | * x == 0) { return Err (UleError :: parse :: < Self > ()) ; } } 1 => U :: validate_bytes (& chunk [1 ..]) ? , _ => return Err (UleError :: parse :: < Self > ()) , } } Ok (()) } }
};
}
