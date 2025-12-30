// Generated macro for impl_4_to_8_bytes (function)
macro_rules! Depcrate_xxhash3_128impl_4_to_8_bytes {
() => {
// Module: crate::xxhash3_128
// Provides: {"impl_4_to_8_bytes"}
// Dependencies: {}
# [inline (always)] fn impl_4_to_8_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u128 { assert_input_range ! (4 ..= 8 , input . len ()) ; let input_first = input . first_u32 () . unwrap () ; let input_last = input . last_u32 () . unwrap () ; let modified_seed = seed ^ (seed . lower_half () . swap_bytes () . into_u64 () << 32) ; let secret_words = secret . for_128 () . words_for_4_to_8 () ; let combined = input_first . into_u64 () | (input_last . into_u64 () << 32) ; let lhs = { let a = secret_words [0] ^ secret_words [1] ; let b = a . wrapping_add (modified_seed) ; b ^ combined } ; let rhs = PRIME64_1 . wrapping_add (input . len () . into_u64 () << 2) ; let mul_result = lhs . into_u128 () . wrapping_mul (rhs . into_u128 ()) ; let mut high = mul_result . upper_half () ; let mut low = mul_result . lower_half () ; high = high . wrapping_add (low << 1) ; low ^= high >> 3 ; low ^= low >> 35 ; low = low . wrapping_mul (PRIME_MX2) ; low ^= low >> 28 ; high = avalanche (high) ; X128 { low , high } . into () }
};
}
