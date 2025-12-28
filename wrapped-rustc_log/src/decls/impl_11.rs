macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl std :: error :: Error for Error { }
    };
}

impl_11!()