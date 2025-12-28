macro_rules! impl_99 {
    () => {
        impl < D : Decoder > Decodable < D > for BytePos { fn decode (d : & mut D) -> BytePos { BytePos (d . read_u32 ()) } }
    };
}

impl_99!()