// Generated macro for impl_698 (impl)
macro_rules! Depcrate_ule_plainimpl_698 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_698"}
// Dependencies: {}
unsafe impl ULE for NonZeroU8 { # [inline] fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { bytes . iter () . try_for_each (| b | { if * b == 0x00 { Err (UleError :: parse :: < Self > ()) } else { Ok (()) } }) } }
};
}
