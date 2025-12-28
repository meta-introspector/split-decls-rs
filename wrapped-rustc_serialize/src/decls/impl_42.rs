macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for path :: PathBuf { fn decode (d : & mut D) -> path :: PathBuf { let bytes : String = Decodable :: decode (d) ; path :: PathBuf :: from (bytes) } }
    };
}

impl_42!()