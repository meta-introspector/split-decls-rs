macro_rules! deps {
    () => {
        IntoData!();
        Data!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl IntoData for & '_ [u8] { fn into_data (self) -> Data { self . to_owned () . into_data () } }
    };
}

impl_125!()