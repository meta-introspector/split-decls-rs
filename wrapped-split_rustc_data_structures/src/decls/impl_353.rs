macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl From < u128 > for Pu128 { # [inline] fn from (value : u128) -> Self { Self (value) } }
    };
}

impl_353!()