macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Error { fn new (kind : ErrorKind) -> Self { Error { kind } } }
    };
}

impl_66!();