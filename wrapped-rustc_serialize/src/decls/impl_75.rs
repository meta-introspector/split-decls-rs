macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for Hash64 { # [inline] fn decode (d : & mut D) -> Self { Self :: new (u64 :: from_le_bytes (d . read_raw_bytes (8) . try_into () . unwrap ())) } }
    };
}

impl_75!()