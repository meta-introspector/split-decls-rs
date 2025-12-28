macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Error { fn new (kind : ErrorKind) -> Self { Error { kind } } }
    };
}

impl_66!()