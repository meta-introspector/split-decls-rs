macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl From < io :: Error > for Error { fn from (err : io :: Error) -> Self { Error :: Io (err) } }
    };
}

impl_74!();