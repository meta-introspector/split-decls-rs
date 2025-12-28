macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl error :: Error for Error { }
    };
}

impl_43!();