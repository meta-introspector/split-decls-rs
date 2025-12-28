macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl From < toml :: ser :: Error > for Error { fn from (err : toml :: ser :: Error) -> Self { Error :: TomlSer (err) } }
    };
}

impl_76!()