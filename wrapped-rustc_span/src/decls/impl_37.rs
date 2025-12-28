macro_rules! deps {
    () => {
        SyntaxContext!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl ! Ord for SyntaxContext { }
    };
}

impl_37!();