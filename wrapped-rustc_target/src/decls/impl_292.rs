macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl ToJson for Json { fn to_json (& self) -> Json { self . clone () } }
    };
}

impl_292!()