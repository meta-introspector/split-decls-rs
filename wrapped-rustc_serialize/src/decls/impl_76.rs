macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for Hash128 { # [inline] fn decode (d : & mut D) -> Self { Self :: new (u128 :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ())) } }
    };
}

impl_76!();