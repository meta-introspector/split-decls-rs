macro_rules! bytes_for_varint {
    () => {
        fn bytes_for_varint (len : NonZeroUsize) -> usize { let usize_bits = mem :: size_of :: < usize > () * 8 ; let len_bits = usize_bits - len . leading_zeros () as usize ; (len_bits + 6) / 7 }
    };
}

bytes_for_varint!();