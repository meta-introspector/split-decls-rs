macro_rules! deps {
    () => {
        X128!();
    };
}

macro_rules! impl_1_to_3_bytes {
    () => {
        deps!();
        # [inline (always)] fn impl_1_to_3_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u128 { assert_input_range ! (1 ..= 3 , input . len ()) ; let combined = impl_1_to_3_bytes_combined (input) ; let secret_words = secret . for_128 () . words_for_1_to_3 () ; let low = { let secret = (secret_words [0] ^ secret_words [1]) . into_u64 () ; secret . wrapping_add (seed) ^ combined . into_u64 () } ; let high = { let secret = (secret_words [2] ^ secret_words [3]) . into_u64 () ; secret . wrapping_sub (seed) ^ combined . swap_bytes () . rotate_left (13) . into_u64 () } ; let low = avalanche_xxh64 (low) ; let high = avalanche_xxh64 (high) ; X128 { low , high } . into () }
    };
}

impl_1_to_3_bytes!();