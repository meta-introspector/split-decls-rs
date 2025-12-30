// Generated macro for avalanche_xxh64 (function)
macro_rules! Depcrate_xxhash3avalanche_xxh64 {
() => {
// Module: crate::xxhash3
// Provides: {"avalanche_xxh64"}
// Dependencies: {}
# [inline] pub fn avalanche_xxh64 (mut x : u64) -> u64 { x ^= x >> 33 ; x = x . wrapping_mul (primes :: PRIME64_2) ; x ^= x >> 29 ; x = x . wrapping_mul (primes :: PRIME64_3) ; x ^= x >> 32 ; x }
};
}
