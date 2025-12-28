macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl std :: error :: Error for SyntaxError { }
    };
}

impl_20!();