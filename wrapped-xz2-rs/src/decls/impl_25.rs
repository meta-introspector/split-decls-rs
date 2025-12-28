macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl error :: Error for Error { }
    };
}

impl_25!();