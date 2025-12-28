macro_rules! deps {
    () => {
        Buffer!();
        Hasher!();
        Accumulators!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Hasher { # [doc = " Hash all data at once. If you can use this function, you may"] # [doc = " see noticable speed gains for certain types of input."] # [must_use] # [inline] pub fn oneshot (seed : u64 , data : & [u8]) -> u64 { let len = data . len () ; let mut accumulators = Accumulators :: new (seed) ; let data = accumulators . write_many (data) ; Self :: finish_with (seed , len . into_u64 () , & accumulators , data) } # [doc = " Constructs the hasher with an initial seed."] # [must_use] pub const fn with_seed (seed : u64) -> Self { Self { seed , accumulators : Accumulators :: new (seed) , buffer : Buffer :: new () , length : 0 , } } # [doc = " The seed this hasher was created with."] pub const fn seed (& self) -> u64 { self . seed } # [doc = " The total number of bytes hashed."] pub const fn total_len (& self) -> u64 { self . length } # [must_use] # [inline] fn finish_with (seed : u64 , len : u64 , accumulators : & Accumulators , mut remaining : & [u8]) -> u64 { let mut acc = if len < BYTES_IN_LANE . into_u64 () { seed . wrapping_add (PRIME64_5) } else { accumulators . finish () } ; acc += len ; while let Some ((chunk , rest)) = remaining . split_first_chunk () { let lane = u64 :: from_ne_bytes (* chunk) . to_le () ; acc ^= round (0 , lane) ; acc = acc . rotate_left (27) . wrapping_mul (PRIME64_1) ; acc = acc . wrapping_add (PRIME64_4) ; remaining = rest ; } while let Some ((chunk , rest)) = remaining . split_first_chunk () { let lane = u32 :: from_ne_bytes (* chunk) . to_le () . into_u64 () ; acc ^= lane . wrapping_mul (PRIME64_1) ; acc = acc . rotate_left (23) . wrapping_mul (PRIME64_2) ; acc = acc . wrapping_add (PRIME64_3) ; remaining = rest ; } for & byte in remaining { let lane = byte . into_u64 () ; acc ^= lane . wrapping_mul (PRIME64_5) ; acc = acc . rotate_left (11) . wrapping_mul (PRIME64_1) ; } acc ^= acc >> 33 ; acc = acc . wrapping_mul (PRIME64_2) ; acc ^= acc >> 29 ; acc = acc . wrapping_mul (PRIME64_3) ; acc ^= acc >> 32 ; acc } }
    };
}

impl_50!();