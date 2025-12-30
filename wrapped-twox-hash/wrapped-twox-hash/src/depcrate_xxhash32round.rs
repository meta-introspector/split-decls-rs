// Generated macro for round (function)
macro_rules! Depcrate_xxhash32round {
() => {
// Module: crate::xxhash32
// Provides: {"round"}
// Dependencies: {}
# [inline] const fn round (mut acc : u32 , lane : u32) -> u32 { acc = acc . wrapping_add (lane . wrapping_mul (PRIME32_2)) ; acc = acc . rotate_left (13) ; acc . wrapping_mul (PRIME32_1) }
};
}
