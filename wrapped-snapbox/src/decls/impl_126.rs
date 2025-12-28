macro_rules! deps {
    () => {
        Data!();
        IntoData!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl IntoData for String { fn into_data (self) -> Data { Data :: text (self) } }
    };
}

impl_126!();