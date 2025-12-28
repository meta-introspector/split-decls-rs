macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for () { fn decode (_ : & mut D) { } }
    };
}

impl_18!();