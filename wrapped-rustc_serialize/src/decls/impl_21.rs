macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Box < [T] > { fn decode (d : & mut D) -> Box < [T] > { let v : Vec < T > = Decodable :: decode (d) ; v . into_boxed_slice () } }
    };
}

impl_21!();