macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl std :: error :: Error for Error { }
    };
}

impl_18!()