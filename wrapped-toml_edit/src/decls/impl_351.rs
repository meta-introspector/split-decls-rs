macro_rules! deps {
    () => {
        Error!();
        TomlError!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl From < Error > for crate :: TomlError { fn from (e : Error) -> Self { Self :: custom (e . to_string () , None) } }
    };
}

impl_351!()