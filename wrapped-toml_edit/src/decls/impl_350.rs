macro_rules! deps {
    () => {
        Error!();
        TomlError!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl From < crate :: TomlError > for Error { fn from (e : crate :: TomlError) -> Self { Self :: custom (e) } }
    };
}

impl_350!();