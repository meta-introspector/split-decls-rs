macro_rules! deps {
    () => {
        IntoData!();
        Data!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl IntoData for String { fn into_data (self) -> Data { Data :: text (self) } }
    };
}

impl_126!()