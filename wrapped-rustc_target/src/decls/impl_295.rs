macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl ToJson for bool { fn to_json (& self) -> Json { Json :: Bool (* self) } }
    };
}

impl_295!()