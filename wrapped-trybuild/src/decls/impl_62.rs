macro_rules! deps {
    () => {
        Directory!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl AsRef < Path > for Directory { fn as_ref (& self) -> & Path { & self . path } }
    };
}

impl_62!()