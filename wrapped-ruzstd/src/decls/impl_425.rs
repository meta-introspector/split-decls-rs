macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl From < ErrorKind > for Error { fn from (value : ErrorKind) -> Self { Self :: from (value) } }
    };
}

impl_425!();