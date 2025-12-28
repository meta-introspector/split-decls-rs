macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for () { fn decode (_ : & mut D) { } }
    };
}

impl_18!()