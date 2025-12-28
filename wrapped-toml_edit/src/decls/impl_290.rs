macro_rules! deps {
    () => {
        TomlError!();
        Error!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl From < crate :: TomlError > for Error { fn from (e : crate :: TomlError) -> Self { Self { inner : e } } }
    };
}

impl_290!();