macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < D : Decoder , T1 : Decodable < D > , T2 : Decodable < D > > Decodable < D > for Result < T1 , T2 > { fn decode (d : & mut D) -> Result < T1 , T2 > { match d . read_u8 () { 0 => Ok (T1 :: decode (d)) , 1 => Err (T2 :: decode (d)) , _ => panic ! ("Encountered invalid discriminant while decoding `Result`.") , } } }
    };
}

impl_36!()