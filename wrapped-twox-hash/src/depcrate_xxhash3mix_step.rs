// Generated macro for mix_step (function)
macro_rules! Depcrate_xxhash3mix_step {
() => {
// Module: crate::xxhash3
// Provides: {"mix_step"}
// Dependencies: {}
# [inline] pub fn mix_step (data : & [u8 ; 16] , secret : & [u8 ; 16] , seed : u64) -> u64 { let data_words = to_u64s (data) ; let secret_words = to_u64s (secret) ; let mul_result = { let a = (data_words [0] ^ secret_words [0] . wrapping_add (seed)) . into_u128 () ; let b = (data_words [1] ^ secret_words [1] . wrapping_sub (seed)) . into_u128 () ; a . wrapping_mul (b) } ; mul_result . lower_half () ^ mul_result . upper_half () }
};
}
