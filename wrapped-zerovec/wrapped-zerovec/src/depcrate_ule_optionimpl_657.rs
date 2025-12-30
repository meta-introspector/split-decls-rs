// Generated macro for impl_657 (impl)
macro_rules! Depcrate_ule_optionimpl_657 {
() => {
// Module: crate::ule::option
// Provides: {"impl_657"}
// Dependencies: {}
unsafe impl < U : VarULE + ? Sized > VarULE for OptionVarULE < U > { # [inline] fn validate_bytes (slice : & [u8]) -> Result < () , UleError > { if slice . is_empty () { return Err (UleError :: length :: < Self > (slice . len ())) ; } # [expect (clippy :: indexing_slicing)] match slice [0] { 0 => { if slice . len () != 1 { Err (UleError :: length :: < Self > (slice . len ())) } else { Ok (()) } } 1 => U :: validate_bytes (& slice [1 ..]) , _ => Err (UleError :: parse :: < Self > ()) , } } # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { let entire_struct_as_slice : * const [u8] = :: core :: ptr :: slice_from_raw_parts (bytes . as_ptr () , bytes . len () - 1) ; & * (entire_struct_as_slice as * const Self) } }
};
}
