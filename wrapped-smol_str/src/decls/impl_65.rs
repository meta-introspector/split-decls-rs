macro_rules! deps {
    () => {
        SmolStr!();
        SmolStrBuilder!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl From < SmolStrBuilder > for SmolStr { fn from (value : SmolStrBuilder) -> Self { value . finish () } }
    };
}

impl_65!()