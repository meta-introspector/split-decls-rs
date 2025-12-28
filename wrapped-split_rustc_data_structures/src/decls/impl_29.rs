macro_rules! deps {
    () => {
        FingerprintComponent!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl FingerprintComponent for Hash64 { # [inline] fn as_u64 (& self) -> u64 { Hash64 :: as_u64 (* self) } }
    };
}

impl_29!()