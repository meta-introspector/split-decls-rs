macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < u32 > for TextSize { # [inline] fn from (raw : u32) -> Self { TextSize { raw } } }
    };
}

impl_23!();