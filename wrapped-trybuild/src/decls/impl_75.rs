macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl From < toml :: de :: Error > for Error { fn from (err : toml :: de :: Error) -> Self { Error :: TomlDe (err) } }
    };
}

impl_75!()