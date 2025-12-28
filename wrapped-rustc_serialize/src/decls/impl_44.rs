macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > + Copy > Decodable < D > for Cell < T > { fn decode (d : & mut D) -> Cell < T > { Cell :: new (Decodable :: decode (d)) } }
    };
}

impl_44!();