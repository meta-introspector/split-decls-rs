macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl core :: error :: Error for Error { }
    };
}

impl_33!()