macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl ToJson for rustc_abi :: CanonAbi { fn to_json (& self) -> Json { self . to_string () . to_json () } }
    };
}

impl_306!()