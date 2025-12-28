macro_rules! deps {
    () => {
        DefaultSecret!();
    };
}

macro_rules! derive_secret {
    () => {
        deps!();
        # [doc = " # Correctness"] # [doc = ""] # [doc = " This function assumes that the incoming buffer has been populated"] # [doc = " with the default secret."] # [inline] pub fn derive_secret (seed : u64 , secret : & mut DefaultSecret) { if seed == DEFAULT_SEED { return ; } let (words , _) = secret . bp_as_chunks_mut () ; let (pairs , _) = words . bp_as_chunks_mut () ; for [a_p , b_p] in pairs { let a = u64 :: from_le_bytes (* a_p) ; let b = u64 :: from_le_bytes (* b_p) ; let a = a . wrapping_add (seed) ; let b = b . wrapping_sub (seed) ; * a_p = a . to_le_bytes () ; * b_p = b . to_le_bytes () ; } }
    };
}

derive_secret!()