macro_rules! deps {
    () => {
        IntoU128!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl IntoU128 for u64 { fn into_u128 (self) -> u128 { u128 :: from (self) } }
    };
}

impl_142!()