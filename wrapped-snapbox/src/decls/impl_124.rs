macro_rules! deps {
    () => {
        Data!();
        IntoData!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl IntoData for Vec < u8 > { fn into_data (self) -> Data { Data :: binary (self) } }
    };
}

impl_124!();