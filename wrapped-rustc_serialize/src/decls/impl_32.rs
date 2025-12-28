macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for Cow < '_ , str > { fn decode (d : & mut D) -> Cow < 'static , str > { let v : String = Decodable :: decode (d) ; Cow :: Owned (v) } }
    };
}

impl_32!()