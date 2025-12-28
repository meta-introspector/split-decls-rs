macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl From < SmolStr > for String { # [inline (always)] fn from (text : SmolStr) -> Self { text . as_str () . into () } }
    };
}

impl_40!()