macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for String { fn decode (d : & mut D) -> String { d . read_str () . to_owned () } }
    };
}

impl_16!()