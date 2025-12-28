macro_rules! deps {
    () => {
        MaybeTempDir!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        impl AsRef < Path > for MaybeTempDir { fn as_ref (& self) -> & Path { self . dir . path () } }
    };
}

impl_596!()