// Generated macro for impl_1_to_3_bytes (function)
macro_rules! Depcrate_xxhash3_64impl_1_to_3_bytes {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_1_to_3_bytes"}
// Dependencies: {}
# [inline (always)] fn impl_1_to_3_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u64 { assert_input_range ! (1 ..= 3 , input . len ()) ; let combined = impl_1_to_3_bytes_combined (input) ; let secret_words = secret . for_64 () . words_for_1_to_3 () ; let value = { let secret = (secret_words [0] ^ secret_words [1]) . into_u64 () ; secret . wrapping_add (seed) ^ combined . into_u64 () } ; avalanche_xxh64 (value) }
};
}
