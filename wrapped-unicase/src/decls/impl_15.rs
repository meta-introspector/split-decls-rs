macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S : AsRef < str > > Hash for Ascii < S > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for byte in self . as_ref () . bytes () . map (| b | b . to_ascii_lowercase ()) { hasher . write_u8 (byte) ; } hasher . write_u8 (0xFF) ; } }
    };
}

impl_15!();