macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'a > ToJson for Cow < 'a , str > { fn to_json (& self) -> Json { Json :: String (self . to_string ()) } }
    };
}

impl_298!()