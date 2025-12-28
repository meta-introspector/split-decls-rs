macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl std :: error :: Error for Error { }
    };
}

impl_187!();