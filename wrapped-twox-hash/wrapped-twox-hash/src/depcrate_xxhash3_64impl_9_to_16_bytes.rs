// Generated macro for impl_9_to_16_bytes (function)
macro_rules! Depcrate_xxhash3_64impl_9_to_16_bytes {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_9_to_16_bytes"}
// Dependencies: {}
# [inline (always)] fn impl_9_to_16_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u64 { assert_input_range ! (9 ..= 16 , input . len ()) ; let input_first = input . first_u64 () . unwrap () ; let input_last = input . last_u64 () . unwrap () ; let secret_words = secret . for_64 () . words_for_9_to_16 () ; let low = ((secret_words [0] ^ secret_words [1]) . wrapping_add (seed)) ^ input_first ; let high = ((secret_words [2] ^ secret_words [3]) . wrapping_sub (seed)) ^ input_last ; let mul_result = low . into_u128 () . wrapping_mul (high . into_u128 ()) ; let value = input . len () . into_u64 () . wrapping_add (low . swap_bytes ()) . wrapping_add (high) . wrapping_add (mul_result . lower_half () ^ mul_result . upper_half ()) ; avalanche (value) }
};
}
