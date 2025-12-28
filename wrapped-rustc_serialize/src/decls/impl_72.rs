macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Arc < [T] > { fn decode (d : & mut D) -> Arc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
    };
}

impl_72!()