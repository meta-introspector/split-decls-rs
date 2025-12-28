macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < T > { fn decode (d : & mut D) -> Arc < T > { Arc :: new (Decodable :: decode (d)) } }
    };
}

impl_48!()