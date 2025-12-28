macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl AsRef < [u8] > for SmolStr { # [inline (always)] fn as_ref (& self) -> & [u8] { self . as_str () . as_bytes () } }
    };
}

impl_29!()