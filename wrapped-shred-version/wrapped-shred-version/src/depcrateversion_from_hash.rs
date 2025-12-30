// Generated macro for version_from_hash (function)
macro_rules! Depcrateversion_from_hash {
() => {
// Module: crate
// Provides: {"version_from_hash"}
// Dependencies: {}
pub fn version_from_hash (hash : & Hash) -> u16 { let hash = hash . as_ref () ; let mut accum = [0u8 ; 2] ; hash . chunks (2) . for_each (| seed | { accum . iter_mut () . zip (seed) . for_each (| (accum , seed) | * accum ^= * seed) }) ; # [allow (clippy :: arithmetic_side_effects)] let version = ((accum [0] as u16) << 8) | accum [1] as u16 ; version . saturating_add (1) }
};
}
