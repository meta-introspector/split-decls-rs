macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! inline_len {
    () => {
        deps!();
        unsafe fn inline_len (repr : & Identifier) -> NonZeroUsize { let repr = unsafe { ptr :: read (repr as * const Identifier as * const NonZeroU64) } ; # [cfg (target_endian = "little")] let zero_bits_on_string_end = repr . leading_zeros () ; # [cfg (target_endian = "big")] let zero_bits_on_string_end = repr . trailing_zeros () ; let nonzero_bytes = 8 - zero_bits_on_string_end as usize / 8 ; unsafe { NonZeroUsize :: new_unchecked (nonzero_bytes) } }
    };
}

inline_len!();