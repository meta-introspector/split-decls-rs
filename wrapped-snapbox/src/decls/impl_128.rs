macro_rules! deps {
    () => {
        Data!();
        IntoData!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl IntoData for & '_ str { fn into_data (self) -> Data { self . to_owned () . into_data () } }
    };
}

impl_128!()