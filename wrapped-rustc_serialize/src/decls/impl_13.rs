macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for NonZero < u32 > { fn decode (d : & mut D) -> Self { NonZero :: new (d . read_u32 ()) . unwrap () } }
    };
}

impl_13!();