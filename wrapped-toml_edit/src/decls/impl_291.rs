macro_rules! deps {
    () => {
        TomlError!();
        Error!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl From < Error > for crate :: TomlError { fn from (e : Error) -> Self { e . inner } }
    };
}

impl_291!()