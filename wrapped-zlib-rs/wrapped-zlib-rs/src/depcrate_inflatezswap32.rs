// Generated macro for zswap32 (function)
macro_rules! Depcrate_inflatezswap32 {
() => {
// Module: crate::inflate
// Provides: {"zswap32"}
// Dependencies: {}
const fn zswap32 (q : u32) -> u32 { u32 :: from_be (q . to_le ()) }
};
}
