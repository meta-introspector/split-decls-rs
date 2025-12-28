macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl core :: error :: Error for ParseError { }
    };
}

impl_23!()