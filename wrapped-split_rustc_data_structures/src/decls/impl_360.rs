macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for Pu128 { # [inline] fn decode (d : & mut D) -> Self { Self (u128 :: decode (d)) } }
    };
}

impl_360!();