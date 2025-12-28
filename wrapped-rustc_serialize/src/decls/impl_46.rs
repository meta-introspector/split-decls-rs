macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for RefCell < T > { fn decode (d : & mut D) -> RefCell < T > { RefCell :: new (Decodable :: decode (d)) } }
    };
}

impl_46!();