// Generated macro for impl_0_bytes (function)
macro_rules! Depcrate_xxhash3_64impl_0_bytes {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_0_bytes"}
// Dependencies: {}
# [inline (always)] fn impl_0_bytes (secret : & Secret , seed : u64) -> u64 { let secret_words = secret . for_64 () . words_for_0 () ; avalanche_xxh64 (seed ^ secret_words [0] ^ secret_words [1]) }
};
}
