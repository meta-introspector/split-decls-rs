macro_rules! impl_1131 {
    () => {
        impl From < OpenptFlags > for OFlags { # [inline] fn from (flags : OpenptFlags) -> Self { Self :: from_bits_retain (flags . bits () as _) } }
    };
}

impl_1131!()