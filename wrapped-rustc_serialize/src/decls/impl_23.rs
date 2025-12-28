macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < T > { fn decode (d : & mut D) -> Rc < T > { Rc :: new (Decodable :: decode (d)) } }
    };
}

impl_23!();