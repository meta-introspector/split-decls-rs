macro_rules! deps {
    () => {
        TempDir!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl AsRef < Path > for TempDir { fn as_ref (& self) -> & Path { self . path () } }
    };
}

impl_5!()