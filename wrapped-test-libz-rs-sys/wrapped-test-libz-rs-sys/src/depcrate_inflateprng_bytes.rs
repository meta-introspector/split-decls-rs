// Generated macro for prng_bytes (function)
macro_rules! Depcrate_inflateprng_bytes {
() => {
// Module: crate::inflate
// Provides: {"prng_bytes"}
// Dependencies: {}
fn prng_bytes (seed : u64 , bytes : & mut [u8] , step : usize) { const M : u64 = 2u64 . pow (32) ; const A : u64 = 1664525 ; const C : u64 = 1013904223 ; let mut state = seed ; for chunk in bytes . chunks_mut (4 * step) { state = (A * state + C) % M ; let rand_bytes = state . to_le_bytes () ; for (i , byte) in chunk . iter_mut () . enumerate () { * byte = rand_bytes [i / step] ; } } }
};
}
