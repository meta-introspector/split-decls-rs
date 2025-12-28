macro_rules! deps {
    () => {
        AstPtr!();
        AstNode!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < N : AstNode > Copy for AstPtr < N > { }
    };
}

impl_8!();