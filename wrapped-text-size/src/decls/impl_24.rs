macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < TextSize > for u32 { # [inline] fn from (value : TextSize) -> Self { value . raw } }
    };
}

impl_24!();