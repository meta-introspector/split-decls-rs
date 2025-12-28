macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl ToJson for rustc_abi :: Endian { fn to_json (& self) -> Json { self . as_str () . to_json () } }
    };
}

impl_305!();