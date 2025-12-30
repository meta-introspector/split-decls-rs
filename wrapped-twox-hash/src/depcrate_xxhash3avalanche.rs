// Generated macro for avalanche (function)
macro_rules! Depcrate_xxhash3avalanche {
() => {
// Module: crate::xxhash3
// Provides: {"avalanche"}
// Dependencies: {}
# [inline] pub fn avalanche (mut x : u64) -> u64 { x ^= x >> 37 ; x = x . wrapping_mul (primes :: PRIME_MX1) ; x ^= x >> 32 ; x }
};
}
