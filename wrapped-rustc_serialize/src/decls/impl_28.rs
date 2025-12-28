macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < D : Decoder , const N : usize > Decodable < D > for [u8 ; N] { fn decode (d : & mut D) -> [u8 ; N] { let len = d . read_usize () ; assert ! (len == N) ; let mut v = [0u8 ; N] ; for i in 0 .. len { v [i] = Decodable :: decode (d) ; } v } }
    };
}

impl_28!()