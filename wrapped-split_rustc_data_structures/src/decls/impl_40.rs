macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for Fingerprint { # [inline] fn decode (d : & mut D) -> Self { Fingerprint :: from_le_bytes (d . read_raw_bytes (16) . try_into () . unwrap ()) } }
    };
}

impl_40!();