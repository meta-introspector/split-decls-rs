macro_rules! deps {
    () => {
        ToJson!();
        LinkerFlavorCli!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl ToJson for LinkerFlavorCli { fn to_json (& self) -> Json { self . desc () . to_json () } }
    };
}

impl_462!()