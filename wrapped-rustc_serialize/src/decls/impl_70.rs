macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Rc < [T] > { fn decode (d : & mut D) -> Rc < [T] > { let vec : Vec < T > = Decodable :: decode (d) ; vec . into () } }
    };
}

impl_70!()