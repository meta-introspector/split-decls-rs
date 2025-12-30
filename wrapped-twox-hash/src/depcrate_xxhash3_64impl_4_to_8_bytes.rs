// Generated macro for impl_4_to_8_bytes (function)
macro_rules! Depcrate_xxhash3_64impl_4_to_8_bytes {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_4_to_8_bytes"}
// Dependencies: {}
# [inline (always)] fn impl_4_to_8_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u64 { assert_input_range ! (4 ..= 8 , input . len ()) ; let input_first = input . first_u32 () . unwrap () ; let input_last = input . last_u32 () . unwrap () ; let modified_seed = seed ^ (seed . lower_half () . swap_bytes () . into_u64 () << 32) ; let secret_words = secret . for_64 () . words_for_4_to_8 () ; let combined = input_last . into_u64 () | (input_first . into_u64 () << 32) ; let mut value = { let a = secret_words [0] ^ secret_words [1] ; let b = a . wrapping_sub (modified_seed) ; b ^ combined } ; value ^= value . rotate_left (49) ^ value . rotate_left (24) ; value = value . wrapping_mul (PRIME_MX2) ; value ^= (value >> 35) . wrapping_add (input . len () . into_u64 ()) ; value = value . wrapping_mul (PRIME_MX2) ; value ^= value >> 28 ; value }
};
}
