// Generated macro for impl_0_bytes (function)
macro_rules! Depcrate_xxhash3_128impl_0_bytes {
() => {
// Module: crate::xxhash3_128
// Provides: {"impl_0_bytes"}
// Dependencies: {}
# [inline (always)] fn impl_0_bytes (secret : & Secret , seed : u64) -> u128 { let secret_words = secret . for_128 () . words_for_0 () ; let low = avalanche_xxh64 (seed ^ secret_words [0] ^ secret_words [1]) ; let high = avalanche_xxh64 (seed ^ secret_words [2] ^ secret_words [3]) ; X128 { low , high } . into () }
};
}
