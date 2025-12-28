macro_rules! deps {
    () => {
        Hasher!();
        Accumulators!();
        Buffer!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Hasher { # [doc = " Hash all data at once. If you can use this function, you may"] # [doc = " see noticable speed gains for certain types of input."] # [must_use] # [inline] pub fn oneshot (seed : u32 , data : & [u8]) -> u32 { let len = data . len () ; let mut accumulators = Accumulators :: new (seed) ; let data = accumulators . write_many (data) ; Self :: finish_with (seed , len . into_u64 () , & accumulators , data) } # [doc = " Constructs the hasher with an initial seed."] # [must_use] pub const fn with_seed (seed : u32) -> Self { Self { seed , accumulators : Accumulators :: new (seed) , buffer : Buffer :: new () , length : 0 , } } # [doc = " The seed this hasher was created with."] pub const fn seed (& self) -> u32 { self . seed } # [doc = " The total number of bytes hashed."] pub const fn total_len (& self) -> u64 { self . length } # [doc = " The total number of bytes hashed, truncated to 32 bits."] # [doc = ""] # [doc = " For the full 64-bit byte count, use [`total_len`](Self::total_len)."] pub const fn total_len_32 (& self) -> u32 { self . length as u32 } # [doc = " Returns the hash value for the values written so far. Unlike"] # [doc = " [`hash::Hasher::finish`][], this method returns the actual 32-bit"] # [doc = " value calculated, not a 64-bit value."] # [must_use] # [inline] pub fn finish_32 (& self) -> u32 { Self :: finish_with (self . seed , self . length , & self . accumulators , self . buffer . remaining () ,) } # [must_use] # [inline] fn finish_with (seed : u32 , len : u64 , accumulators : & Accumulators , mut remaining : & [u8]) -> u32 { let mut acc = if len < BYTES_IN_LANE . into_u64 () { seed . wrapping_add (PRIME32_5) } else { accumulators . finish () } ; acc += len as u32 ; while let Some ((chunk , rest)) = remaining . split_first_chunk () { let lane = u32 :: from_ne_bytes (* chunk) . to_le () ; acc = acc . wrapping_add (lane . wrapping_mul (PRIME32_3)) ; acc = acc . rotate_left (17) . wrapping_mul (PRIME32_4) ; remaining = rest ; } for & byte in remaining { let lane = byte . into_u32 () ; acc = acc . wrapping_add (lane . wrapping_mul (PRIME32_5)) ; acc = acc . rotate_left (11) . wrapping_mul (PRIME32_1) ; } acc ^= acc >> 15 ; acc = acc . wrapping_mul (PRIME32_2) ; acc ^= acc >> 13 ; acc = acc . wrapping_mul (PRIME32_3) ; acc ^= acc >> 16 ; acc } }
    };
}

impl_21!()