macro_rules! deps {
    () => {
        X128!();
    };
}

macro_rules! impl_9_to_16_bytes {
    () => {
        deps!();
        # [inline (always)] fn impl_9_to_16_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u128 { assert_input_range ! (9 ..= 16 , input . len ()) ; let input_first = input . first_u64 () . unwrap () ; let input_last = input . last_u64 () . unwrap () ; let secret_words = secret . for_128 () . words_for_9_to_16 () ; let val1 = ((secret_words [0] ^ secret_words [1]) . wrapping_sub (seed)) ^ input_first ^ input_last ; let val2 = ((secret_words [2] ^ secret_words [3]) . wrapping_add (seed)) ^ input_last ; let mul_result = val1 . into_u128 () . wrapping_mul (PRIME64_1 . into_u128 ()) ; let low = mul_result . lower_half () . wrapping_add ((input . len () - 1) . into_u64 () << 54) ; let high = mul_result . upper_half () . wrapping_add (val2 . upper_half () . into_u64 () << 32) . wrapping_add (val2 . lower_half () . into_u64 () . wrapping_mul (PRIME32_2)) ; let low = low ^ high . swap_bytes () ; let q = X128 { low , high } . into_u128 () . wrapping_mul (PRIME64_2 . into_u128 ()) ; let low = avalanche (q . lower_half ()) ; let high = avalanche (q . upper_half ()) ; X128 { low , high } . into () }
    };
}

impl_9_to_16_bytes!()