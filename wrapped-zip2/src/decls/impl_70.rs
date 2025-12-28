macro_rules! deps {
    () => {
        ZipResult!();
        UnicodeExtraField!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl UnicodeExtraField { pub (crate) fn try_from_reader < R : Read > (reader : & mut R , len : u16) -> ZipResult < Self > { reader . read_exact (& mut [0u8]) ? ; let crc32 = reader . read_u32_le () ? ; let content_len = (len as usize) . checked_sub (size_of :: < u8 > () + size_of :: < u32 > ()) . ok_or (invalid ! ("Unicode extra field is too small")) ? ; let mut content = vec ! [0u8 ; content_len] . into_boxed_slice () ; reader . read_exact (& mut content) ? ; Ok (Self { crc32 , content }) } }
    };
}

impl_70!()