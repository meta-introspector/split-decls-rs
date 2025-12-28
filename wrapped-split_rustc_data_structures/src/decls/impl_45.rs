macro_rules! deps {
    () => {
        PackedFingerprint!();
        Fingerprint!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl From < Fingerprint > for PackedFingerprint { # [inline] fn from (f : Fingerprint) -> PackedFingerprint { PackedFingerprint (f) } }
    };
}

impl_45!()