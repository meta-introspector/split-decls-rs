macro_rules! deps {
    () => {
        CompilerError!();
        Error!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < T > std :: error :: Error for CompilerError < T > where T : Display + Debug { }
    };
}

impl_280!()