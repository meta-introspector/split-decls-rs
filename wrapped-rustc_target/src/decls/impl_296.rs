macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl ToJson for str { fn to_json (& self) -> Json { Json :: String (self . to_owned ()) } }
    };
}

impl_296!();