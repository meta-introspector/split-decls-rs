macro_rules! avalanche_xxh64 {
    () => {
        # [inline] pub fn avalanche_xxh64 (mut x : u64) -> u64 { x ^= x >> 33 ; x = x . wrapping_mul (primes :: PRIME64_2) ; x ^= x >> 29 ; x = x . wrapping_mul (primes :: PRIME64_3) ; x ^= x >> 32 ; x }
    };
}

avalanche_xxh64!();