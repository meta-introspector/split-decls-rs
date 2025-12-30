// Generated macro for round (function)
macro_rules! Depcrate_xxhash64round {
() => {
// Module: crate::xxhash64
// Provides: {"round"}
// Dependencies: {}
# [inline] const fn round (mut acc : u64 , lane : u64) -> u64 { acc = acc . wrapping_add (lane . wrapping_mul (PRIME64_2)) ; acc = acc . rotate_left (31) ; acc . wrapping_mul (PRIME64_1) }
};
}
