macro_rules! deps {
    () => {
        PackedFingerprint!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < E : Encoder > Encodable < E > for PackedFingerprint { # [inline] fn encode (& self , s : & mut E) { let copy = self . 0 ; copy . encode (s) ; } }
    };
}

impl_43!();