macro_rules! deps {
    () => {
        IntoData!();
        Data!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl IntoData for & '_ String { fn into_data (self) -> Data { self . to_owned () . into_data () } }
    };
}

impl_127!();