macro_rules! deps {
    () => {
        X128!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl From < X128 > for u128 { fn from (value : X128) -> Self { value . high . into_u128 () << 64 | value . low . into_u128 () } }
    };
}

impl_84!()