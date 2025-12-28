macro_rules! deps {
    () => {
        Fingerprint!();
        PackedFingerprint!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < D : Decoder > Decodable < D > for PackedFingerprint { # [inline] fn decode (d : & mut D) -> Self { Self (Fingerprint :: decode (d)) } }
    };
}

impl_44!();