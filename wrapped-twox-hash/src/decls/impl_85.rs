macro_rules! deps {
    () => {
        X128!();
        IntoU128!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl crate :: IntoU128 for X128 { fn into_u128 (self) -> u128 { self . into () } }
    };
}

impl_85!();