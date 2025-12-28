macro_rules! impl_102 {
    () => {
        impl < D : Decoder > Decodable < D > for RelativeBytePos { fn decode (d : & mut D) -> RelativeBytePos { RelativeBytePos (d . read_u32 ()) } }
    };
}

impl_102!()