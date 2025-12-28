macro_rules! deps {
    () => {
        FatalError!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl std :: error :: Error for FatalError { }
    };
}

impl_228!();