macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl ToJson for Align { fn to_json (& self) -> Json { self . bits () . to_json () } }
    };
}

impl_521!();