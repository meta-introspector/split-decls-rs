macro_rules! impl_326 {
    () => {
        impl < D : Decoder > Decodable < D > for RelativeBytePos { fn decode (d : & mut D) -> RelativeBytePos { RelativeBytePos (d . read_u32 ()) } }
    };
}

impl_326!();