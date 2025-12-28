macro_rules! deps {
    () => {
        IntoData!();
        Data!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl IntoData for Data { fn into_data (self) -> Data { self } }
    };
}

impl_122!()