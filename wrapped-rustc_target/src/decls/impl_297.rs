macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl ToJson for String { fn to_json (& self) -> Json { Json :: String (self . to_owned ()) } }
    };
}

impl_297!();