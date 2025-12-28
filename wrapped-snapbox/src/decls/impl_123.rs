macro_rules! deps {
    () => {
        Data!();
        IntoData!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl IntoData for & '_ Data { fn into_data (self) -> Data { self . clone () } }
    };
}

impl_123!();