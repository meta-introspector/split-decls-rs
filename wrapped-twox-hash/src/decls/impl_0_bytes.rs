macro_rules! deps {
    () => {
        X128!();
    };
}

macro_rules! impl_0_bytes {
    () => {
        deps!();
        # [inline (always)] fn impl_0_bytes (secret : & Secret , seed : u64) -> u128 { let secret_words = secret . for_128 () . words_for_0 () ; let low = avalanche_xxh64 (seed ^ secret_words [0] ^ secret_words [1]) ; let high = avalanche_xxh64 (seed ^ secret_words [2] ^ secret_words [3]) ; X128 { low , high } . into () }
    };
}

impl_0_bytes!();