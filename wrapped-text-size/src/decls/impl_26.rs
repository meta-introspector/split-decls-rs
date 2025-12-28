macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < TextSize > for usize { # [inline] fn from (value : TextSize) -> Self { value . raw as usize } }
    };
}

impl_26!();