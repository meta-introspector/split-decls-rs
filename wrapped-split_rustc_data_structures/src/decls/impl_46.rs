macro_rules! deps {
    () => {
        PackedFingerprint!();
        Fingerprint!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl From < PackedFingerprint > for Fingerprint { # [inline] fn from (f : PackedFingerprint) -> Fingerprint { f . 0 } }
    };
}

impl_46!()