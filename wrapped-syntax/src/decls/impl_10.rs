macro_rules! deps {
    () => {
        AstNode!();
        AstPtr!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < N : AstNode > Eq for AstPtr < N > { }
    };
}

impl_10!()