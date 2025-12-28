macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < T > { fn decode (d : & mut D) -> Box < T > { Box :: new (Decodable :: decode (d)) } }
    };
}

impl_50!();