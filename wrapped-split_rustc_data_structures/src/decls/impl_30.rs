macro_rules! deps {
    () => {
        FingerprintComponent!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl FingerprintComponent for u64 { # [inline] fn as_u64 (& self) -> u64 { * self } }
    };
}

impl_30!()