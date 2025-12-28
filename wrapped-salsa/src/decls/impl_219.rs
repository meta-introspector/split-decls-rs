macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl Lookup < PathBuf > for & Path { fn into_owned (self) -> PathBuf { self . to_owned () } }
    };
}

impl_219!();