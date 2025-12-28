macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > + ToOwned > Decodable < D > for Cow < 'static , [T] > where [T] : ToOwned < Owned = Vec < T > > , { fn decode (d : & mut D) -> Cow < 'static , [T] > { let v : Vec < T > = Decodable :: decode (d) ; Cow :: Owned (v) } }
    };
}

impl_30!()