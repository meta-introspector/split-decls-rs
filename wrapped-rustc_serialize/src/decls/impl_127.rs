macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
        MemDecoder!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a > Decodable < MemDecoder < 'a > > for Vec < u8 > { fn decode (d : & mut MemDecoder < 'a >) -> Self { let len = Decoder :: read_usize (d) ; d . read_raw_bytes (len) . to_owned () } }
    };
}

impl_127!()