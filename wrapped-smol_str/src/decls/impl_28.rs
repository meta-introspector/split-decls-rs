macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl AsRef < str > for SmolStr { # [inline (always)] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_28!()