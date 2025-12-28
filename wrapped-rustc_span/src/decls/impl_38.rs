macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl ! PartialOrd for SyntaxContext { }
    };
}

impl_38!()